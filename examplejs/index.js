const {
  Account,
  Connection,
  PublicKey,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} = require("@solana/web3.js");
const axios = require("axios");
const BufferLayout = require("buffer-layout");

// constants
const simplePriceDBDataLayout = BufferLayout.struct([
  BufferLayout.blob(32, "owner"),
  BufferLayout.blob(8, "latest_symbol"),
  BufferLayout.blob(8, "latest_price"),
]);
const connection = new Connection("https://devnet.solana.com", "singleGossip");
const payerAccount = new Account(
  Buffer.from(process.env.PAYER_SECRET_KEY, "hex")
);
const stdReferencePubkey = new PublicKey(
  "GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM"
);
const simplePriceDBPubkey = new PublicKey(
  "5ZMhT6afgRs39ty1R2bLyoZBLuP7o3gsmrSYenaxHRNw"
);
const simplePriceDBProgramId = new PublicKey(
  "8quk86nFqUBTzNxhwqoBHgTkG6MYkw9nsWXuCqg1vQw9"
);

const sleep = async (ms) => new Promise((r) => setTimeout(r, ms));

const setPrice = async (symbol) => {
  const payerPubkey = payerAccount.publicKey;
  console.log(
    "Set the latest price for simple price by payer ",
    payerPubkey.toBase58()
  );

  const instruction = new TransactionInstruction({
    keys: [
      { pubkey: simplePriceDBPubkey, isSigner: false, isWritable: true },
      { pubkey: payerPubkey, isSigner: true, isWritable: true },
      { pubkey: stdReferencePubkey, isSigner: false, isWritable: false },
    ],
    programId: simplePriceDBProgramId,
    data: Buffer.concat([
      Buffer.from([2]),
      Buffer.from(symbol, "utf-8"),
      Buffer.from(Array(8 - symbol.length).fill(0)),
    ]),
  });
  const txHash = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [payerAccount],
    {
      commitment: "singleGossip",
      preflightCommitment: "singleGossip",
    }
  );
  console.log("txHash: ", txHash);
};

const getCurrentPriceFromSimplePriceDB = async () => {
  try {
    const accountInfo = await connection.getAccountInfo(simplePriceDBPubkey);
    if (accountInfo === null) {
      console.log("Error: cannot find the greeted account");
      return;
    }
    const { owner, latest_symbol, latest_price } =
      simplePriceDBDataLayout.decode(Buffer.from(accountInfo.data));
    console.log("    owner: ", new PublicKey(owner).toBase58());
    console.log("    latest_symbol: ", latest_symbol.toString());
    console.log("    latest_price: ", latest_price.readBigInt64LE().toString());
  } catch (e) {
    console.log("Fail to get rate from std contract");
    console.log(e);
  }
};

(async () => {
  try {
    const symbol = process.argv[2];

    console.log("current state of simple price db");
    await getCurrentPriceFromSimplePriceDB();
    console.log("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");

    console.log("set symbol = ", symbol);
    await setPrice(symbol);
    await sleep(5000);
    console.log("-------------------------------");

    console.log("new state of simple price db");
    await getCurrentPriceFromSimplePriceDB();
    console.log("===============================");
  } catch (e) {
    console.log(e);
  }
})();

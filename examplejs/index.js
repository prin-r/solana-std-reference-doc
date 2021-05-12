const axios = require("axios");

// constants
const secretKey = process.env.SECRET_KEY;
const GAS = 200_000;

// band constants
const bandUrl = "https://asia-rpc.bandchain.org/oracle/request_prices";
const symbols = ["BTC", "ETH", "BAND"];

const sleep = async (ms) => new Promise((r) => setTimeout(r, ms));

const getPricesFromBand = async () => {
  try {
    const {
      data: { result },
    } = await axios.post(bandUrl, {
      symbols,
      min_count: 10,
      ask_count: 16,
    });
    return {
      symbols,
      rates: result.map((e) => e["px"] + ""),
      resolve_times: result.map((e) => Number(e["resolve_time"])),
      request_ids: result.map((e) => Number(e["request_id"])),
    };
  } catch (e) {
    console.log(e);
    return null;
  }
};

const validateTx = async (txhash) => {
  let max_retry = 30;
  while (max_retry > 0) {
    await sleep(1000);
    max_retry--;
    try {
      process.stdout.clearLine();
      process.stdout.cursorTo(0);
      process.stdout.write("polling: " + (30 - max_retry));
      // pass
      // return ...;
    } catch (err) {
      if (err.isAxiosError && err.response && err.response.status !== 404) {
        console.error(err.response.data);
      } else if (!err.isAxiosError) {
        console.error(err.message);
      }
    }
  }
  return null;
};

const getCurrentRateFromStdContract = async () => {
  try {
    // const result = await ...
    // return result;
  } catch (e) {
    console.log("Fail to get rate from std contract");
    console.log(e);
  }
  return null;
};

(async () => {
  while (true) {
    try {
      // get prices from band
      const relay = await getPricesFromBand();
      if (relay) {
        console.log("\nrelay message: ", JSON.stringify({ relay }));
      } else {
        throw "Fail to get stock price from band";
      }

      // broadcast tx
      // const { txhash } = await ...
      console.log("broadcast tx: ", txhash);

      // wait for tx result
      const txResult = await validateTx(txhash);
      console.log("\n");
      if (!txResult) {
        throw "Fail to get result from chain";
      }

      if (!txResult.code) {
        console.log("tx successfully send!");
      } else {
        throw "Fail to send tx with result: " + JSON.stringify(txResult);
      }

      // get rates from std_reference_basic
      const currentRates = await getCurrentRateFromStdContract();
      if (currentRates) {
        console.log("current rates: ", JSON.stringify(currentRates));
      } else {
        throw "Fail to get current rates from std contract";
      }
    } catch (e) {
      console.log(e);
    }
    console.log(
      "=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-="
    );
    await sleep(10_000);
  }
})();

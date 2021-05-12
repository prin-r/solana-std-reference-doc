# Band Protocol Solana Developer Documentation

In addition to data native to the [Solana blockchain](https://docs.solana.com/), Solana developers also have access to various cryptocurrency price data provided by [Band Protocol](https://bandprotocol.com/)'s oracle.

## Standard Reference Dataset Contract Info

### Data Available

The price data originates from [data requests](https://github.com/bandprotocol/bandchain/wiki/System-Overview#oracle-data-request) made on BandChain and then sent to Band's [std_reference_basic](https://explorer.solana.com/address/GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM?cluster=devnet) contract on Solana then retrieves and stores the results of those requests. Specifically, the following price pairs are available to be read from the the contract:

For example

- AAPL/USD
- GOOGL/USD
- TSLA/USD

These prices are automatically updated every 5 minutes. The [std_reference_basic](https://explorer.solana.com/address/GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM?cluster=devnet) itself is currently deployed on Solana devnet at [`GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM`](https://explorer.solana.com/address/GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM?cluster=devnet).

The prices themselves are the median of the values retrieved by BandChain's validators from many sources including [CoinGecko](https://www.coingecko.com/api/documentations/v3), [CryptoCompare](https://min-api.cryptocompare.com/), [Binance](https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md), [CoinMarketcap](https://coinmarketcap.com/), [HuobiPro](https://www.huobi.vc/en-us/exchange/), [CoinBasePro](https://pro.coinbase.com/), [Kraken](https://www.kraken.com/), [Bitfinex](https://www.bitfinex.com/), [Bittrex](https://global.bittrex.com/), [BITSTAMP](https://www.bitstamp.net/), [OKEX](https://www.okex.com/), [FTX](https://ftx.com/), [HitBTC](https://hitbtc.com/), [ItBit](https://www.itbit.com/), [Bithumb](https://www.bithumb.com/), [CoinOne](https://coinone.co.kr/). The data request is then made by executing Band's aggregator oracle script, the code of which you can view on Band's [mainnet](https://cosmoscan.io/oracle-script/3). Along with the price data, developers will also have access to the latest timestamp the price was updated.

These parameters are intended to act as security parameters to help anyone using the data to verify that the data they are using is what they expect and, perhaps more importantly, actually valid.

### Standard Reference Dataset Contract Price Update Process

For the ease of development, the Band Foundation will be maintaining and updating the [std_reference_basic](https://explorer.solana.com/address/GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM?cluster=devnet) contract with the latest price data. In the near future, we will be releasing guides on how developers can create similar contracts themselves to retrieve data from Band's oracle.

## Retrieving the Price Data

The code below shows an example of a relatively [simple_price_database](https://explorer.solana.com/address/5ZMhT6afgRs39ty1R2bLyoZBLuP7o3gsmrSYenaxHRNw?cluster=devnet) contract on Solana which retrieve price data from Band's [std_reference_basic](https://explorer.solana.com/address/GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM?cluster=devnet) contract and store it in the contract's state. This following diagram shows the working steps of a message `SetPrice` only, which will be explain on the next section.

```shell=
       (1) Send message "SetPrice"
        |
        | [std_reference_basic account]
        | [symbol]
        v
===================
|                 |<-----------
| simple price db |           | (4) Extract price from raw data
|                 |------------     and then save to the state
===================
 |               ^
 |(2)            |(3)
 |Ask for        |
 |account's for  |Retrive the account's data
 |data           |
 v               |
=========================
|                       |
|  std_reference_basic  |
|                       |
=========================

```

The contract is able to store exchange rate of any price pair that available on the [std_reference_basic](https://explorer.solana.com/address/GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM?cluster=devnet) contract. For more information on what oracle scripts are and how data requests work on BandChain in general, please see their [wiki](https://github.com/bandprotocol/bandchain/wiki/System-Overview#oracle-data-request) and [developer documentation](https://docs.bandchain.org/dapp-developers/requesting-data-from-bandchain)

## Code Breakdown

Now we are going to breakdown the [simple_price_database](https://explorer.solana.com/address/5ZMhT6afgRs39ty1R2bLyoZBLuP7o3gsmrSYenaxHRNw?cluster=devnet) contract. The contract can be broken down into 4 sections which are `state`, `struct types`, `commands`.

#### State

```rust
pub struct SimplePriceDB {
    owner: [u8; 32],
    latest_symbol: [u8; 8],
    latest_price: u64,
}
```

The full code of state is [here](./simple_price_db/src/lib.rs#27).

The state of this contract only contain 3 variables which are `owner`, `latest_symbol` and `latest_price`. The `latest_symbol` and `latest_price` are set by function `SetPrice`, and their set value is the most recent value from calling `SetPrice`.

#### Struct Types

The full code of struct types is [here](./simple_price_db/src/lib.rs#12).

1. Price is a helper struct of StdReferenceBasic.

```rust
pub struct Price {
    symbol: [u8; 8],
    rate: u64,
    last_updated: u64,
    request_id: u64,
}
```

2. StdReferenceBasic is an interface that help parsing the data from [std_reference_basic](https://explorer.solana.com/address/GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM?cluster=devnet).

```rust
pub struct StdReferenceBasic {
    owner: [u8; 32],
    current_size: u8,
    prices: Vec<Price>,
}
```

#### Commands

The full code of contract logic is [here](./simple_price_db/src/lib.rs#60).

The [simple_price_database](https://explorer.solana.com/address/5ZMhT6afgRs39ty1R2bLyoZBLuP7o3gsmrSYenaxHRNw?cluster=devnet) only consist of 3 commands which are `Init`, `TransferOwnership` and `SetPrice`.

1. `Init` is a function that can only be called once. We will be able to call this function when we have just finished our contract deployment.

```rust
Command::Init(owner) => {
    msg!("Init!");
    let proxy_account = next_account_info(account_info_iter)?;
    let temp = (*proxy_account.data.borrow()).to_vec();

    // Error if the contract is initialized
    if is_initialized(&temp) {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Convert raw account's data in to SimplePriceDB struct
    let mut spdb = SimplePriceDB::try_from_slice(&temp).map_err(|_| ProgramError::Custom(113))?;

    // Set owner
    spdb.owner = owner;

    // Set latest_symbol and latest_price to zero values
    spdb.latest_symbol = [0u8; 8];
    spdb.latest_price = 0u64;

    // Save change
    spdb.serialize(&mut &mut proxy_account.data.borrow_mut()[..])?;

    Ok(())
}
```

2. `TransferOwnership` allow the current owner to transfer control of the contract to a new owner. The input parameter is a pubkey of the new owner.

```rust
Command::TransferOwnership(new_owner) => {
    msg!("TransferOwnership!");
    let simple_price_db = next_account_info(account_info_iter)?;
    let sender = next_account_info(account_info_iter)?;
    if !sender.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let temp = (*simple_price_db.data.borrow()).to_vec();

    // error if not initialized
    if !is_initialized(&temp) {
        return Err(ProgramError::UninitializedAccount);
    }

    // parse raw account's data into SimplePriceDB struct
    let mut spdb = SimplePriceDB::try_from_slice(&temp).map_err(|_| ProgramError::Custom(113))?;

    // check owner
    if spdb.owner != sender.key.to_bytes() {
        return Err(ProgramError::Custom(112));
    }

    // set owner
    spdb.owner = new_owner;

    // save change
    spdb.serialize(&mut &mut simple_price_db.data.borrow_mut()[..])?;
    Ok(())
}
```

3. `SetPrice` read through the from [std_reference_basic](https://explorer.solana.com/address/GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM?cluster=devnet) and use it to find the price for a particular symbol. The input parameter is a symbol that the sender want to set.

```rust
Command::SetPrice(symbol) => {
    msg!("SetPrice!");
    let simple_price_db = next_account_info(account_info_iter)?;
    let sender = next_account_info(account_info_iter)?;
    let std_reference_account = next_account_info(account_info_iter)?;

    // check that sender is signer
    if !sender.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let temp = (*simple_price_db.data.borrow()).to_vec();

    // error if not initialized
    if !is_initialized(&temp) {
        return Err(ProgramError::UninitializedAccount);
    }

    // parse raw account's data into SimplePriceDB struct
    let mut spdb = SimplePriceDB::try_from_slice(&temp).map_err(|_| ProgramError::Custom(113))?;

    // check owner
    if spdb.owner != sender.key.to_bytes() {
        return Err(ProgramError::Custom(112));
    }

    // get std_reference's state
    let temp2 = (*std_reference_account.data.borrow()).to_vec();
    let std_reference = StdReferenceBasic::try_from_slice(&temp2).map_err(|_| ProgramError::Custom(113))?;

    // filter only a price of the symbol
    let rate = std_reference.prices.iter().find(|&p| p.symbol == symbol).map_or(None, |p| Some(p.rate));
    if rate.is_none() {
        msg!("Price not found !");
        return Err(ProgramError::Custom(115));
    }

    // set latest price and symbol
    spdb.latest_price = rate.unwrap();
    spdb.latest_symbol = symbol;

    // save change
    spdb.serialize(&mut &mut simple_price_db.data.borrow_mut()[..])?;
    Ok(())
}
```

## List of Band oracle contracts on Solana networks.

### Solana Devnet Contracts

| Contract        |                  Public Key                  |
| --------------- | :------------------------------------------: |
| std_reference   | GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM |
| simple_price_db | 5ZMhT6afgRs39ty1R2bLyoZBLuP7o3gsmrSYenaxHRNw |

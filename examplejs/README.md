# Example js of interactions with Simple Price DB

This example demonstrates how to set a price in the [simple_price_database](https://explorer.solana.com/address/5ZMhT6afgRs39ty1R2bLyoZBLuP7o3gsmrSYenaxHRNw?cluster=devnet) contract by using `@solana/web3.js`.
Start by asking for the state of the [simple_price_database](https://explorer.solana.com/address/5ZMhT6afgRs39ty1R2bLyoZBLuP7o3gsmrSYenaxHRNw?cluster=devnet) first and then setting the price by sending a transaction to Solana devnet. At the end it asks for the [simple_price_database](https://explorer.solana.com/address/5ZMhT6afgRs39ty1R2bLyoZBLuP7o3gsmrSYenaxHRNw?cluster=devnet)'s state one more time to show that the state has changed after the price has been set.

### node version

`using node v13.14.0`

### install

`yarn`

### setup your env

`export PAYER_SECRET_KEY=<YOUR_PAYER_SECRET_KEY>`

### set price for each symbol

- `node index.js BTC`
- `node index.js ETH`

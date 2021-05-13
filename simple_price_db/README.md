# Simple Price DB Contract

### Address on devnet

program id: [8quk86nFqUBTzNxhwqoBHgTkG6MYkw9nsWXuCqg1vQw9](https://explorer.solana.com/address/8quk86nFqUBTzNxhwqoBHgTkG6MYkw9nsWXuCqg1vQw9?cluster=devnet)

state account: [5ZMhT6afgRs39ty1R2bLyoZBLuP7o3gsmrSYenaxHRNw](https://explorer.solana.com/address/5ZMhT6afgRs39ty1R2bLyoZBLuP7o3gsmrSYenaxHRNw?cluster=devnet)

### Building

This project cannot be built directly via cargo and instead requires the build scripts located in Solana's BPF-SDK.

Build the project directly via:

```console
cargo build-bpf --manifest-path=./Cargo.toml --bpf-out-dir=./
```

After the building is complete, you will see `solana_bpf_simple_price_db.so ` appear in this dir.

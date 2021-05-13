# Std Reference Basic Contract

### Address on devnet

program id: [7q9ELgzyJL2dawNfsocYdgtB7cJiNBo8kgeR7ZjjbVk](https://explorer.solana.com/address/7q9ELgzyJL2dawNfsocYdgtB7cJiNBo8kgeR7ZjjbVk?cluster=devnet)

state account: [GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM](https://explorer.solana.com/address/GcSkynL9Emy5fPVnhR2rEJJjiXwGKGa4euSdPZEGRFHM?cluster=devnet)

### Building

This project cannot be built directly via cargo and instead requires the build scripts located in Solana's BPF-SDK.

Build the project directly via:

```console
cargo build-bpf --manifest-path=./Cargo.toml --bpf-out-dir=./
```

After the building is complete, you will see `solana_bpf_simple_price_db.so ` appear in this dir.

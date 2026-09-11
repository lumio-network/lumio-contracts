# lumio-contracts

**The trust layer.** Soroban smart contracts on [Stellar](https://stellar.org) that put a
savings group's ledger on-chain: pooled contributions, governance, payouts, and votes become
visible to every member and provable by the chain.

Part of [Lumio](https://github.com/lumio-network) — an open-source cooperative finance platform
for savings groups, cooperatives, and community investment clubs (*ajo*, *esusu*, *chamas*,
table-banking groups, SACCOs).

> ⚠️ **Scaffold / pre-audit.** These contracts expose minimal stub logic so the workspace builds
> and tests green. They are **not** complete, **not** audited, and must not custody real funds.
> Full logic lands in a later phase.

## Contracts

| Contract     | Status                       | Responsibility                                             |
| ------------ | ---------------------------- | ---------------------------------------------------------- |
| `treasury`   | Build first — "live, tested" | Holds pooled funds, records contributions/withdrawals.     |
| `governance` | Build second                 | Proposal creation, member voting rules.                    |
| `dividends`  | Build third                  | Calculates and distributes payouts to members.             |
| `voting`     | Build fourth                 | Vote casting/tallying logic used by governance.            |

Each contract is an independent crate under [`contracts/`](./contracts), sharing one Cargo
workspace.

## Requirements

- Rust (stable) with the `wasm32v1-none` target — see [`rust-toolchain.toml`](./rust-toolchain.toml).
- [`soroban-sdk`](https://crates.io/crates/soroban-sdk) `27.x` (pinned in the workspace).
- Optional: the [Stellar CLI](https://developers.stellar.org/docs/tools/cli) for deploys.

## Getting started

```bash
# from a fresh clone — no other setup needed
cargo build --workspace      # or: make build
cargo test  --workspace      # or: make test
```

Other shortcuts (see [`Makefile`](./Makefile)):

```bash
make fmt          # format
make fmt-check    # verify formatting (what CI runs)
make wasm         # optimized wasm for every contract
```

## Layout

```
lumio-contracts/
├── contracts/
│   ├── treasury/   { src/lib.rs, src/test.rs, Cargo.toml }
│   ├── governance/
│   ├── dividends/
│   └── voting/
├── Cargo.toml            # workspace root
├── Makefile              # build/test/fmt/wasm shortcuts
├── rust-toolchain.toml
└── .github/workflows/ci.yml
```

## License

[Apache-2.0](./LICENSE).

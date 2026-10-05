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

## Entrypoints

Public `#[contractimpl]` methods of each contract. `env: Env` is implicit on every call and omitted
below. Signatures mirror the code; update this table by hand whenever a contract's API changes.

| Contract | Entrypoint | Signature | Description |
| --- | --- | --- | --- |
| `treasury` | `deposit` | `(member: Address, amount: i128) -> Result<i128, Error>` | Records a contribution from `member`; returns their new balance. Rejects `amount <= 0`. |
| `treasury` | `balance` | `(member: Address) -> i128` | Amount `member` has contributed so far (`0` if none). |
| `treasury` | `total` | `() -> i128` | Total pooled across every member (`0` if none). |
| `governance` | `create_proposal` | `(proposer: Address, title: String) -> Result<u32, Error>` | Creates an open proposal and returns its id. Requires `proposer` auth; rejects an empty title. |
| `governance` | `close_proposal` | `(id: u32) -> Result<(), Error>` | Closes a proposal. Fails if it does not exist or is already closed. |
| `governance` | `get_proposal` | `(id: u32) -> Option<Proposal>` | Fetches a stored proposal by id, if it exists. |
| `governance` | `proposal_count` | `() -> u32` | Number of proposals created so far (`0` if none). |
| `dividends` | `fund` | `(amount: i128) -> Result<i128, Error>` | Adds `amount` to the distributable pool; returns the new pool balance. Rejects `amount <= 0`. |
| `dividends` | `record_share` | `(member: Address, amount: i128) -> Result<i128, Error>` | Records a payout share for `member`; returns their new share. Rejects `amount <= 0`. |
| `dividends` | `share_of` | `(member: Address) -> i128` | Payout share recorded for `member` (`0` if none). |
| `dividends` | `pool` | `() -> i128` | Current distributable pool balance (`0` if unfunded). |
| `dividends` | `total_shares` | `() -> i128` | Running sum of all recorded shares across every member (`0` if none). |
| `voting` | `set_governance` | `(governance: Address)` | Sets the governance contract used to validate proposals before votes are recorded. |
| `voting` | `governance` | `() -> Option<Address>` | The configured governance contract address, if any. |
| `voting` | `cast_vote` | `(proposal_id: u32, voter: Address, approve: bool) -> Result<(), Error>` | Casts one vote (`approve = true` counts as yes). Requires `voter` auth; fails if the voter already voted, or if governance is set and the proposal is missing or closed. |
| `voting` | `tally` | `(proposal_id: u32) -> Tally` | Yes/no tallies for `proposal_id` (both `0` if no votes). |

## Error codes

Contract errors are returned as `Error(Contract, #N)`. Codes are scoped to each contract.

| Contract | Error variant | Code |
| --- | --- | ---: |
| `treasury` | `InvalidAmount` | 1 |
| `treasury` | `Overflow` | 2 |
| `governance` | `ProposalNotFound` | 1 |
| `governance` | `EmptyTitle` | 2 |
| `governance` | `AlreadyClosed` | 3 |
| `dividends` | `InvalidAmount` | 1 |
| `dividends` | `Overflow` | 2 |
| `voting` | `AlreadyVoted` | 1 |
| `voting` | `ProposalNotFound` | 2 |
| `voting` | `ProposalClosed` | 3 |

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

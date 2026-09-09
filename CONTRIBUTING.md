# Contributing to lumio-contracts

Thanks for your interest in Lumio — an open-source cooperative finance platform for savings
groups (*ajo*, *esusu*, *chamas*, SACCOs) on Stellar / Soroban. This repo holds the **on-chain
layer**: the Soroban smart contracts. It pairs with the
[SDK](https://github.com/lumio-network/lumio-sdk) and the
[apps](https://github.com/lumio-network/lumio-app).

Small, focused pull requests are very welcome — including your first one.

> ⚠️ **Scaffold / pre-audit.** These contracts expose minimal stub logic so the workspace builds
> and tests green. They are **not** complete, **not** audited, and must not custody real funds.
> Protocol logic that's explicitly deferred to a later phase — auth (`require_auth`), token
> transfers, quorum/threshold rules, vote weighting, and pro-rata payout math — is **out of scope**
> for contributor PRs unless an issue says otherwise. Good starting points are correctness guards,
> idiomatic-Soroban error handling, and tests on the existing entrypoints.

## What's in here

One Cargo workspace; each contract is an independent crate under `contracts/`:

| Crate | Responsibility |
| --- | --- |
| `treasury` | Records pooled member contributions and a running total. |
| `governance` | Stores proposals with incrementing ids. |
| `dividends` | Funds a pool and records per-member payout shares. |
| `voting` | One-address-one-vote casting and yes/no tallies. |

## Prerequisites

- **Rust (stable)** with the **`wasm32v1-none`** target — see [`rust-toolchain.toml`](./rust-toolchain.toml).
  (`soroban-sdk` 27 requires `wasm32v1-none` on Rust 1.84+; the older `wasm32-unknown-unknown`
  is rejected by its build script. `rustup` will install the target automatically from the toolchain file.)
- **`soroban-sdk` 27.0.6**, pinned in the workspace `Cargo.toml`.
- Optional: the [Stellar CLI](https://developers.stellar.org/docs/tools/cli) for deploys.

## Getting started

```bash
# from a fresh clone — no other setup needed
cargo build --workspace      # or: make build
cargo test  --workspace      # or: make test
```

Shortcuts (see [`Makefile`](./Makefile)):

```bash
make fmt          # format
make fmt-check    # verify formatting (what CI runs)
make wasm         # optimized wasm for every contract (--target wasm32v1-none --release)
```

## Before you open a PR

CI runs, in order: **format check → build → test → release wasm build** (see
[`.github/workflows/ci.yml`](./.github/workflows/ci.yml)) and must be green. Run the equivalents locally:

```bash
cargo fmt --all -- --check
cargo build --workspace --locked
cargo test  --workspace --locked
cargo build --workspace --locked --target wasm32v1-none --release
```

## Making a change

1. **Find or open an issue.** Browse [`good first issue`](https://github.com/lumio-network/lumio-contracts/labels/good%20first%20issue)
   and [`help wanted`](https://github.com/lumio-network/lumio-contracts/labels/help%20wanted), and
   comment on the one you'd like so we can assign it.
2. **Branch** off `main` (e.g. `fix/voting-contracterror`).
3. **Keep it focused.** One issue per PR; add or update a `#[test]` (each crate has a `src/test.rs`)
   for the behaviour you change.
4. **Commit** using [Conventional Commits](https://www.conventionalcommits.org/), e.g.
   `fix(voting): return a structured error on double vote`.
5. **Open the PR** against `main`, link the issue (`Closes #123`), and describe what changed and how
   you verified it (paste `cargo test` output).

## Reporting a bug or proposing work

Open an issue with a clear title, the affected crate, and — for a behaviour bug — a minimal test or
`cargo`-level reproduction. Because this is pre-audit scaffold code, please **do not** file
speculative "full protocol" feature requests; scope to a single, testable change.

Found a security concern? Please do not open a public issue with exploit details — see
[`SECURITY.md`](./SECURITY.md) if present, or contact the maintainers privately via the org.

## License

By contributing you agree that your contributions are licensed under the project's
[Apache-2.0](./LICENSE) license.

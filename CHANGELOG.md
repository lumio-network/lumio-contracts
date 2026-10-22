# Changelog

All notable changes to this project are documented in this file.
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

_No changes yet. Add an entry here when work lands between releases._

## [0.1.0] - 2026-09-07

First scaffold of the `lumio-contracts` workspace: the on-chain layer for the
Lumio cooperative-finance platform, shipped as four independent Soroban crates
under one Cargo workspace (all pinned to `soroban-sdk` 27.0.6, Apache-2.0). Each
crate is a minimal, build- and test-green stub, not audited and not ready to
custody funds.

### Added

- **Workspace scaffolding** — a single Cargo workspace with four member crates,
  a `release` profile tuned for small deterministic Soroban wasm, and a
  `release-with-logs` variant that keeps debug assertions for debugging deploys.
- **`treasury`** — `deposit` / `balance` / `total` bookkeeping for pooled member
  contributions, with `i128` overflow guards on the running totals.
- **`governance`** — `create_proposal` / `close_proposal` / `get_proposal` /
  `proposal_count` for proposals with incrementing ids, with `ProposalCreated`
  and `ProposalClosed` event types and structured error codes.
- **`dividends`** — `fund` / `record_share` / `share_of` / `pool` /
  `total_shares` for a distributable pool and per-member payout shares, with
  `i128` overflow guards.
- **`voting`** — `set_governance` / `governance` / `cast_vote` / `tally`
  implementing one-address-one-vote casting with yes/no tallies, a `VoteCast`
  event, and an optional cross-contract guard that checks the referenced
  proposal exists and is open when a governance address is configured.
- **CI and tooling** — a `Makefile` (`build` / `test` / `fmt` / `wasm`), a
  clippy lint gate plus a Rust beta matrix leg in CI, and a contributor guide
  in `CONTRIBUTING.md`.

### Notes

- Scaffold / pre-audit: the full protocol behaviour — token transfers, quorum
  and threshold rules, vote weighting, and pro-rata payout math — is
  intentionally stubbed or deferred to a later phase and is out of scope for
  contributor PRs unless an issue says otherwise. See the warning at the top of
  `CONTRIBUTING.md`.
> This repository is a **scaffold / pre-audit** workspace. Every release below
> ships minimal, testable stub logic so the workspace builds and tests green.
> Protocol behaviour explicitly deferred to a later phase (auth, token
> transfers, quorum/threshold rules, vote weighting, pro-rata payout math)
> is intentionally **not** part of any of these releases.

## [Unreleased]

_No changes yet._

## [0.1.0] - 2026-10-05

Initial scaffold of the Lumio on-chain workspace: one Cargo workspace with
four independent Soroban contract crates, plus a deterministic wasm release
profile. Everything is testable but intentionally incomplete.

### Added

- **`contracts/treasury`** — records pooled member contributions and keeps a
  running total.
- **`contracts/governance`** — stores proposals with incrementing ids.
- **`contracts/dividends`** — funds a pool and records per-member payout
  shares.
- **`contracts/voting`** — one-address-one-vote casting with yes/no tallies.
- Workspace pinned to `soroban-sdk 27.0.6` and the `wasm32v1-none` target
  (see `rust-toolchain.toml`).
- Deterministic release profile (`opt-level = "z"`, `lto`, `panic = "abort"`,
  symbols stripped) and a `release-with-logs` variant that keeps debug
  assertions.
- CI pipeline: format check → build → test → release wasm build
  (`.github/workflows/ci.yml`), plus committed test snapshots under
  `contracts/<crate>/test_snapshots/`.

### Known limitations

- All four contracts expose minimal stub logic. They are **not** complete and
  **not** audited, and must not custody real funds.
- Auth, token transfers, quorum/threshold rules, vote weighting, and pro-rata
  payout math are deferred to later phases and are out of scope for
  contributor PRs unless an issue says otherwise.

[Unreleased]: https://github.com/lumio-network/lumio-contracts/compare/0.1.0...HEAD
[0.1.0]: https://github.com/lumio-network/lumio-contracts/releases/tag/0.1.0

---
name: Feature request
about: Propose a new entrypoint, guard, or improvement to a contract
title: "feat(<contract>): <short description>"
labels: enhancement
assignees: ""
---

## Which contract / crate?

<!-- treasury / governance / dividends / voting / workspace / docs -->

## Problem or motivation

<!-- What gap or limitation does this address? Reference the module doc ("later-phase" items) if relevant. -->

## Proposed solution

<!-- Describe the change at the entrypoint level. E.g.:
- Add `withdraw(env, member, amount)` to treasury that decrements balance and total.
- Reject `amount <= 0` with a typed `#[contracterror]`. -->

## Acceptance criteria

- [ ] <!-- measurable criterion 1 -->
- [ ] <!-- measurable criterion 2 -->
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo build --workspace` passes
- [ ] `cargo test --workspace` passes

## Out of scope

<!-- What this issue deliberately does NOT cover. -->

## Additional context

<!-- Links to Soroban docs, related issues, prior art, etc. -->

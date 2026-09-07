#![no_std]
//! Lumio **treasury** contract — the pooled vault (scaffold).
//!
//! Design target (later phase): a multisig-controlled group wallet where members
//! contribute on a schedule and withdrawals require threshold approval from the
//! `governance` contract. This scaffold records per-member contributions and a
//! running total in contract storage so the crate builds and tests green.
//!
//! Not audited. Do not custody real funds.

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Total contributed by a single member.
    Balance(Address),
    /// Running total across all members.
    Total,
}

#[contract]
pub struct TreasuryContract;

#[contractimpl]
impl TreasuryContract {
    /// Record a contribution of `amount` from `member` into the pooled treasury.
    /// Returns the member's new recorded balance.
    ///
    /// Scaffold: pure bookkeeping — no token transfer or auth is wired yet.
    pub fn deposit(env: Env, member: Address, amount: i128) -> i128 {
        let key = DataKey::Balance(member);
        let prev: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        let next = prev + amount;
        env.storage().persistent().set(&key, &next);

        let total: i128 = env.storage().instance().get(&DataKey::Total).unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::Total, &(total + amount));

        next
    }

    /// The amount `member` has contributed so far.
    pub fn balance(env: Env, member: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(member))
            .unwrap_or(0)
    }

    /// The total pooled across every member.
    pub fn total(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Total).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;

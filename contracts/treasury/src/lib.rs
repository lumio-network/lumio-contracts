#![no_std]
//! Lumio **treasury** contract — the pooled vault (scaffold).
//!
//! Design target (later phase): a multisig-controlled group wallet where members
//! contribute on a schedule and withdrawals require threshold approval from the
//! `governance` contract. This scaffold records per-member contributions and a
//! running total in contract storage so the crate builds and tests green.
//!
//! Not audited. Do not custody real funds.

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
    InsufficientBalance = 2,
}

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
    /// Rejects non-positive amounts (amount must be > 0).
    /// Emits event: topic=("deposit", member), data=amount
    pub fn deposit(env: Env, member: Address, amount: i128) -> Result<i128, Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let key = DataKey::Balance(member.clone());
        let prev: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        let next = prev + amount;
        env.storage().persistent().set(&key, &next);

        let total: i128 = env.storage().instance().get(&DataKey::Total).unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::Total, &(total + amount));

        env.events().publish(("deposit", member), amount);

        Ok(next)
    }

    /// Withdraw `amount` from `member`'s recorded balance in the pooled treasury.
    /// Returns the member's new recorded balance.
    ///
    /// Rejects non-positive amounts and withdrawals greater than the member's balance.
    /// Emits event: topic=("withdraw", member), data=amount
    pub fn withdraw(env: Env, member: Address, amount: i128) -> Result<i128, Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let key = DataKey::Balance(member.clone());
        let prev: i128 = env.storage().persistent().get(&key).unwrap_or(0);

        if amount > prev {
            return Err(Error::InsufficientBalance);
        }

        let next = prev - amount;
        env.storage().persistent().set(&key, &next);

        let total: i128 = env.storage().instance().get(&DataKey::Total).unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::Total, &(total - amount));

        env.events().publish(("withdraw", member), amount);

        Ok(next)
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

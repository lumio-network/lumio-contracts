#![no_std]
//! Lumio **dividends** contract — payout calculation and distribution (scaffold).
//!
//! Design target (later phase): given the treasury's surplus and each member's
//! participation, compute pro-rata shares and distribute them on-chain. This
//! scaffold funds a pool and records per-member shares so the crate builds and
//! tests green.
//!
//! Not audited.

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Payout share recorded for a member.
    Share(Address),
    /// Distributable pool balance.
    Pool,
}

#[contract]
pub struct DividendsContract;

#[contractimpl]
impl DividendsContract {
    /// Add `amount` to the distributable pool. Returns the new pool balance.
    /// Rejects non-positive amounts (amount must be > 0).
    pub fn fund(env: Env, amount: i128) -> Result<i128, Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let pool: i128 = env.storage().instance().get(&DataKey::Pool).unwrap_or(0);
        let next = pool + amount;
        env.storage().instance().set(&DataKey::Pool, &next);
        Ok(next)
    }

    /// Record a payout share of `amount` for `member`. Returns their new share.
    ///
    /// Scaffold: pro-rata calculation and on-chain transfer arrive in a later phase.
    /// Rejects non-positive amounts (amount must be > 0).
    pub fn record_share(env: Env, member: Address, amount: i128) -> Result<i128, Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let key = DataKey::Share(member);
        let prev: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        let next = prev + amount;
        env.storage().persistent().set(&key, &next);
        Ok(next)
    }

    /// The payout share currently recorded for `member`.
    pub fn share_of(env: Env, member: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Share(member))
            .unwrap_or(0)
    }

    /// The current distributable pool balance.
    pub fn pool(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Pool).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;

#![no_std]
//! Lumio **voting** contract — vote casting and tallying (scaffold).
//!
//! Design target (later phase): the tally engine invoked by `governance` — vote
//! weighting, eligibility, and windows enforced there. This scaffold implements
//! one-address-one-vote with yes/no tallies so the crate builds and tests green.
//!
//! Not audited.

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyVoted = 1,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Yes tally for a proposal.
    Yes(u32),
    /// No tally for a proposal.
    No(u32),
    /// Whether an address has already voted on a proposal.
    Voted(u32, Address),
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    /// Cast a vote on `proposal_id` by `voter` (`approve = true` counts as yes).
    ///
    /// Scaffold: enforces one vote per address; weighting and eligibility checks
    /// arrive in a later phase. Returns an error if the voter has already voted.
    pub fn cast_vote(
        env: Env,
        proposal_id: u32,
        voter: Address,
        approve: bool,
    ) -> Result<(), Error> {
        let voted_key = DataKey::Voted(proposal_id, voter);
        let already: bool = env.storage().persistent().get(&voted_key).unwrap_or(false);
        if already {
            return Err(Error::AlreadyVoted);
        }
        env.storage().persistent().set(&voted_key, &true);

        let tally_key = if approve {
            DataKey::Yes(proposal_id)
        } else {
            DataKey::No(proposal_id)
        };
        let count: u32 = env.storage().persistent().get(&tally_key).unwrap_or(0);
        env.storage().persistent().set(&tally_key, &(count + 1));
        Ok(())
    }

    /// Return the `(yes, no)` tallies for `proposal_id`.
    pub fn tally(env: Env, proposal_id: u32) -> (u32, u32) {
        let yes: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::Yes(proposal_id))
            .unwrap_or(0);
        let no: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::No(proposal_id))
            .unwrap_or(0);
        (yes, no)
    }
}

#[cfg(test)]
mod test;

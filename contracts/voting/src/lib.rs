#![no_std]
//! Lumio **voting** contract — vote casting and tallying (scaffold).
//!
//! Design target (later phase): the tally engine invoked by `governance` — vote
//! weighting, eligibility, and windows enforced there. This scaffold implements
//! one-address-one-vote with yes/no tallies so the crate builds and tests green.
//!
//! Not audited.

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

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
    /// arrive in a later phase. Panics if the voter has already voted.
    /// Emits event: topic=("cast_vote", voter, proposal_id), data=approve
    pub fn cast_vote(env: Env, proposal_id: u32, voter: Address, approve: bool) {
        let voted_key = DataKey::Voted(proposal_id, voter.clone());
        let already: bool = env.storage().persistent().get(&voted_key).unwrap_or(false);
        if already {
            panic!("voter has already voted on this proposal");
        }
        env.storage().persistent().set(&voted_key, &true);

        let tally_key = if approve {
            DataKey::Yes(proposal_id)
        } else {
            DataKey::No(proposal_id)
        };
        let count: u32 = env.storage().persistent().get(&tally_key).unwrap_or(0);
        env.storage().persistent().set(&tally_key, &(count + 1));

        env.events()
            .publish(("cast_vote", voter, proposal_id), approve);
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

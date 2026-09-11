#![no_std]
//! Lumio **governance** contract — proposals and voting rules (scaffold).
//!
//! Design target (later phase): members open proposals ("buy a maize mill", "raise
//! monthly dues"), the contract enforces quorum/threshold rules, and tallying is
//! delegated to the `voting` contract. This scaffold stores proposals with
//! incrementing ids so the crate builds and tests green.
//!
//! Not audited.

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ProposalNotFound = 1,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Number of proposals created so far (also the latest id).
    ProposalCount,
    /// A stored proposal by id.
    Proposal(u32),
}

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u32,
    pub proposer: Address,
    pub title: String,
    pub open: bool,
}

#[contract]
pub struct GovernanceContract;

#[contractimpl]
impl GovernanceContract {
    /// Create a proposal authored by `proposer`. Returns the new proposal id.
    ///
    /// Requires authorization from `proposer`.
    /// Scaffold: quorum, voting windows arrive in a later phase.
    pub fn create_proposal(env: Env, proposer: Address, title: String) -> u32 {
        proposer.require_auth();

        let id: u32 = env
            .storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0)
            + 1;

        let proposal = Proposal {
            id,
            proposer,
            title,
            open: true,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Proposal(id), &proposal);
        env.storage().instance().set(&DataKey::ProposalCount, &id);

        id
    }

    /// Close a proposal by setting its `open` flag to false.
    /// Returns an error if the proposal does not exist.
    ///
    /// Scaffold: quorum/threshold logic and voting windows arrive in a later phase.
    pub fn close_proposal(env: Env, id: u32) -> Result<(), Error> {
        let key = DataKey::Proposal(id);
        let mut proposal: Proposal = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::ProposalNotFound)?;

        proposal.open = false;
        env.storage().persistent().set(&key, &proposal);

        Ok(())
    }

    /// Fetch a stored proposal by id, if it exists.
    pub fn get_proposal(env: Env, id: u32) -> Option<Proposal> {
        env.storage().persistent().get(&DataKey::Proposal(id))
    }

    /// How many proposals have been created.
    pub fn proposal_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test;

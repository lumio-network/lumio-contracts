#![no_std]
//! Lumio **voting** contract — vote casting and tallying (scaffold).
//!
//! Design target (later phase): the tally engine invoked by `governance` — vote
//! weighting, eligibility, and windows enforced there. This scaffold implements
//! one-address-one-vote with yes/no tallies so the crate builds and tests green.
//!
//! When a governance contract address is configured via `set_governance`, every
//! `cast_vote` call cross-checks that the proposal exists **and** is open before
//! recording the tally. Without a configured governance address the check is
//! skipped (backwards-compatible behaviour).
//!
//! Not audited.

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyVoted = 1,
    ProposalNotFound = 2,
    ProposalClosed = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tally {
    pub yes: u32,
    pub no: u32,
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
    /// Optional governance contract address used to validate proposals.
    GovernanceAddress,
}

#[contractevent]
pub struct VoteCast {
    #[topic]
    pub proposal_id: u32,
    #[topic]
    pub voter: Address,
    pub approve: bool,
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    /// Configure (or update) the governance contract address used to validate
    /// proposals before votes are recorded.
    ///
    /// When set, `cast_vote` will cross-call `get_proposal` on the governance
    /// contract and reject votes on missing or closed proposals.
    pub fn set_governance(env: Env, governance: Address) {
        env.storage()
            .instance()
            .set(&DataKey::GovernanceAddress, &governance);
    }

    /// Return the configured governance contract address, if any.
    pub fn governance(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::GovernanceAddress)
    }

    /// Cast a vote on `proposal_id` by `voter` (`approve = true` counts as yes).
    ///
    /// Requires authorization from `voter`.
    /// Scaffold: enforces one vote per address; weighting and eligibility checks
    /// arrive in a later phase. Returns an error if the voter has already voted.
    pub fn cast_vote(
        env: Env,
        proposal_id: u32,
        voter: Address,
        approve: bool,
    ) -> Result<(), Error> {
        voter.require_auth();

        if let Some(governance) = env.storage().instance().get(&DataKey::GovernanceAddress) {
            let client = lumio_governance::GovernanceContractClient::new(&env, &governance);
            let proposal = client
                .get_proposal(&proposal_id)
                .ok_or(Error::ProposalNotFound)?;
            if !proposal.open {
                return Err(Error::ProposalClosed);
            }
        }

        let voted_key = DataKey::Voted(proposal_id, voter.clone());
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
        env.events().publish_event(&VoteCast {
            proposal_id,
            voter,
            approve,
        });
        Ok(())
    }

    /// Return the yes and no tallies for `proposal_id`.
    pub fn tally(env: Env, proposal_id: u32) -> Tally {
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
        Tally { yes, no }
    }
}

#[cfg(test)]
mod test;

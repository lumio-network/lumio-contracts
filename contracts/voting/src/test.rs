#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

// ── helpers ──────────────────────────────────────────────────────────────────

/// Register both contracts and wire the voting contract to governance.
fn setup_with_governance(env: &Env) -> (VotingContractClient, Address) {
    let gov_id = env.register(lumio_governance::GovernanceContract, ());
    let voting_id = env.register(VotingContract, ());
    let vclient = VotingContractClient::new(env, &voting_id);
    vclient.set_governance(&gov_id);
    (vclient, gov_id)
}

// ── existing tests (must stay green) ─────────────────────────────────────────

#[test]
fn cast_vote_tallies_yes_and_no() {
    let env = Env::default();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    let a = Address::generate(&env);
    let b = Address::generate(&env);
    let c = Address::generate(&env);

    assert_eq!(client.tally(&1), (0, 0));

    client.cast_vote(&1, &a, &true);
    client.cast_vote(&1, &b, &true);
    client.cast_vote(&1, &c, &false);

    assert_eq!(client.tally(&1), (2, 1));
}

#[test]
#[should_panic(expected = "already voted")]
fn double_voting_panics() {
    let env = Env::default();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    let a = Address::generate(&env);
    client.cast_vote(&1, &a, &true);
    client.cast_vote(&1, &a, &false);
}

// ── cross-contract guard tests ────────────────────────────────────────────────

#[test]
fn cast_vote_allowed_on_open_proposal_with_governance() {
    let env = Env::default();
    let (vclient, gov_id) = setup_with_governance(&env);

    // Create an open proposal via the governance contract
    let gclient = lumio_governance::GovernanceContractClient::new(&env, &gov_id);
    let proposer = Address::generate(&env);
    let proposal_id = gclient.create_proposal(&proposer, &String::from_str(&env, "Test"));

    let voter = Address::generate(&env);
    vclient.cast_vote(&proposal_id, &voter, &true);

    assert_eq!(vclient.tally(&proposal_id), (1, 0));
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn cast_vote_rejected_on_missing_proposal_with_governance() {
    let env = Env::default();
    let (vclient, _gov_id) = setup_with_governance(&env);

    let voter = Address::generate(&env);
    // Proposal 99 was never created
    vclient.cast_vote(&99, &voter, &true);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn cast_vote_rejected_on_closed_proposal_with_governance() {
    use lumio_governance::{DataKey as GovDataKey, Proposal};

    let env = Env::default();
    let (vclient, gov_id) = setup_with_governance(&env);

    let gclient = lumio_governance::GovernanceContractClient::new(&env, &gov_id);
    let proposer = Address::generate(&env);
    let proposal_id = gclient.create_proposal(&proposer, &String::from_str(&env, "Close me"));

    // Force-close the proposal by writing a closed copy directly into storage
    env.as_contract(&gov_id, || {
        let key = GovDataKey::Proposal(proposal_id);
        let mut p: Proposal = env.storage().persistent().get(&key).unwrap();
        p.open = false;
        env.storage().persistent().set(&key, &p);
    });

    let voter = Address::generate(&env);
    vclient.cast_vote(&proposal_id, &voter, &true);
}

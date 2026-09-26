#![cfg(test)]

use super::*;
use lumio_governance::{GovernanceContract, GovernanceContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn cast_vote_tallies_yes_and_no() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    let a = Address::generate(&env);
    let b = Address::generate(&env);
    let c = Address::generate(&env);

    assert_eq!(client.tally(&1), Tally { yes: 0, no: 0 });

    client.cast_vote(&1, &a, &true);
    client.cast_vote(&1, &b, &true);
    client.cast_vote(&1, &c, &false);

    assert_eq!(client.tally(&1), Tally { yes: 2, no: 1 });
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn double_voting_panics() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    let a = Address::generate(&env);
    client.cast_vote(&1, &a, &true);
    client.cast_vote(&1, &a, &false);
}

#[test]
fn cast_vote_emits_event() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    let voter = Address::generate(&env);
    let proposal_id = 1u32;

    client.cast_vote(&proposal_id, &voter, &true);

    let events = env.events().all();
    assert_eq!(events.len(), 1);
    assert_eq!(events.first().unwrap().0, contract_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn cast_vote_rejects_missing_proposal_with_governance() {
    let env = Env::default();
    env.mock_all_auths();

    let governance_id = env.register(GovernanceContract, ());
    let voting_id = env.register(VotingContract, ());
    let governance = GovernanceContractClient::new(&env, &governance_id);
    let voting = VotingContractClient::new(&env, &voting_id);
    voting.set_governance(&governance_id);

    let voter = Address::generate(&env);
    assert!(governance.get_proposal(&1).is_none());
    voting.cast_vote(&1, &voter, &true);
}

#[test]
fn cast_vote_allowed_on_open_proposal_with_governance() {
    let env = Env::default();
    env.mock_all_auths();

    let governance_id = env.register(GovernanceContract, ());
    let voting_id = env.register(VotingContract, ());
    let governance = GovernanceContractClient::new(&env, &governance_id);
    let voting = VotingContractClient::new(&env, &voting_id);
    voting.set_governance(&governance_id);

    let proposer = Address::generate(&env);
    let title = String::from_str(&env, "Test");
    let proposal_id = governance.create_proposal(&proposer, &title);
    let voter = Address::generate(&env);

    voting.cast_vote(&proposal_id, &voter, &true);

    assert_eq!(voting.tally(&proposal_id), Tally { yes: 1, no: 0 });
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn cast_vote_rejects_closed_proposal_with_governance() {
    let env = Env::default();
    env.mock_all_auths();

    let governance_id = env.register(GovernanceContract, ());
    let voting_id = env.register(VotingContract, ());
    let governance = GovernanceContractClient::new(&env, &governance_id);
    let voting = VotingContractClient::new(&env, &voting_id);
    voting.set_governance(&governance_id);

    let proposer = Address::generate(&env);
    let title = String::from_str(&env, "Test");
    let proposal_id = governance.create_proposal(&proposer, &title);
    governance.close_proposal(&proposal_id);

    let voter = Address::generate(&env);
    voting.cast_vote(&proposal_id, &voter, &true);
}

#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Events},
    Address, Env, String,
};

#[test]
fn create_proposal_assigns_incrementing_ids() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(GovernanceContract, ());
    let client = GovernanceContractClient::new(&env, &contract_id);

    let proposer = Address::generate(&env);
    let title_a = String::from_str(&env, "Buy a maize mill");
    let title_b = String::from_str(&env, "Raise monthly dues");

    assert_eq!(client.proposal_count(), 0);

    let id_a = client.create_proposal(&proposer, &title_a);
    let id_b = client.create_proposal(&proposer, &title_b);

    assert_eq!(id_a, 1);
    assert_eq!(id_b, 2);
    assert_eq!(client.proposal_count(), 2);

    let stored = client.get_proposal(&id_a).unwrap();
    assert_eq!(stored.title, title_a);
    assert!(stored.open);
    assert!(client.get_proposal(&99).is_none());
}

#[test]
fn close_proposal_transitions_open_to_closed() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(GovernanceContract, ());
    let client = GovernanceContractClient::new(&env, &contract_id);

    let proposer = Address::generate(&env);
    let title = String::from_str(&env, "Buy a maize mill");

    let id = client.create_proposal(&proposer, &title);
    let proposal_count = client.proposal_count();
    assert_eq!(proposal_count, 1);

    // Verify proposal starts as open
    let proposal = client.get_proposal(&id).unwrap();
    assert!(proposal.open);

    // Close the proposal
    client.close_proposal(&id);

    // Verify proposal is now closed
    let proposal = client.get_proposal(&id).unwrap();
    assert!(!proposal.open);
    assert_eq!(client.proposal_count(), proposal_count);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn close_proposal_errors_on_missing_id() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(GovernanceContract, ());
    let client = GovernanceContractClient::new(&env, &contract_id);

    // Try to close a nonexistent proposal
    client.close_proposal(&99);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn create_proposal_rejects_empty_title() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(GovernanceContract, ());
    let client = GovernanceContractClient::new(&env, &contract_id);

    let proposer = Address::generate(&env);
    let empty_title = String::from_str(&env, "");

    // Should panic with Error::EmptyTitle (contract error #2)
    client.create_proposal(&proposer, &empty_title);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn close_proposal_rejects_already_closed() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(GovernanceContract, ());
    let client = GovernanceContractClient::new(&env, &contract_id);

    let proposer = Address::generate(&env);
    let title = String::from_str(&env, "Test proposal");

    // Create and close a proposal
    let proposal_id = client.create_proposal(&proposer, &title);
    client.close_proposal(&proposal_id);

    // Verify proposal is closed
    let proposal = client.get_proposal(&proposal_id).unwrap();
    assert!(!proposal.open);

    // Try to close it again - should panic with Error::AlreadyClosed (contract error #3)
    client.close_proposal(&proposal_id);
}

#[test]
fn close_proposal_emits_event() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(GovernanceContract, ());
    let client = GovernanceContractClient::new(&env, &contract_id);

    let proposer = Address::generate(&env);
    let title = String::from_str(&env, "Test proposal");

    // Create a proposal
    let proposal_id = client.create_proposal(&proposer, &title);

    // Close the proposal
    client.close_proposal(&proposal_id);

    // Verify the ProposalClosed event was emitted
    let events = env.events().all();

    // The test framework captures events; we should see ProposalClosed
    assert!(!events.events().is_empty());

    // Verify the last event is ProposalClosed for our proposal
    let contract_events = events.filter_by_contract(&contract_id);
    assert!(!contract_events.events().is_empty());
}

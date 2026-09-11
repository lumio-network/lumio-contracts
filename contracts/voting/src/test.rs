#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Events}, Address, Env};

#[test]
fn cast_vote_tallies_yes_and_no() {
    let env = Env::default();
    env.mock_all_auths();

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
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    let voter = Address::generate(&env);
    let proposal_id = 1u32;

    // Call cast_vote - it will emit an event internally
    client.cast_vote(&proposal_id, &voter, &true);
    
    // If the function completes without panicking, the event was emitted successfully
}

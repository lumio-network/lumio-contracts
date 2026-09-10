#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

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
#[should_panic(expected = "Error(Contract, #1)")]
fn double_voting_panics() {
    let env = Env::default();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    let a = Address::generate(&env);
    client.cast_vote(&1, &a, &true);
    client.cast_vote(&1, &a, &false);
}

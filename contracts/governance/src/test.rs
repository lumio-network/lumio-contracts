#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn create_proposal_assigns_incrementing_ids() {
    let env = Env::default();
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

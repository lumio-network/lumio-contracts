#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn deposit_accumulates_and_reports_balances() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    assert_eq!(client.balance(&alice), 0);
    assert_eq!(client.total(), 0);

    assert_eq!(client.deposit(&alice, &100), 100);
    assert_eq!(client.deposit(&alice, &50), 150);
    assert_eq!(client.deposit(&bob, &25), 25);

    assert_eq!(client.balance(&alice), 150);
    assert_eq!(client.balance(&bob), 25);
    assert_eq!(client.total(), 175);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn deposit_rejects_negative_amount() {
    let env = Env::default();
    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);
    client.deposit(&alice, &-100);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn deposit_rejects_zero_amount() {
    let env = Env::default();
    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);
    client.deposit(&alice, &0);
}

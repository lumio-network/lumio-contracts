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
fn deposits_keep_member_balances_isolated_and_total_consistent() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);

    client.deposit(&alice, &100);
    client.deposit(&bob, &25);
    client.deposit(&charlie, &70);
    client.deposit(&alice, &15);
    client.deposit(&bob, &10);

    let alice_balance = client.balance(&alice);
    let bob_balance = client.balance(&bob);
    let charlie_balance = client.balance(&charlie);

    assert_eq!(alice_balance, 115);
    assert_eq!(bob_balance, 35);
    assert_eq!(charlie_balance, 70);
    assert_eq!(client.total(), 220);
    assert_eq!(
        client.total(),
        alice_balance + bob_balance + charlie_balance
    );
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

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn deposit_returns_overflow_error_on_i128_max() {
    let env = Env::default();
    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);
    // Fill alice's balance to i128::MAX, then adding 1 must overflow.
    client.deposit(&alice, &i128::MAX);
    client.deposit(&alice, &1);
}

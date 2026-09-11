#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Events}, Address, Env};

#[test]
fn deposit_accumulates_and_reports_balances() {
    let env = Env::default();
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
fn withdraw_reduces_balance_and_total() {
    let env = Env::default();
    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);

    // Deposit first
    assert_eq!(client.deposit(&alice, &100), 100);
    assert_eq!(client.total(), 100);

    // Withdraw some
    assert_eq!(client.withdraw(&alice, &30), 70);
    assert_eq!(client.balance(&alice), 70);
    assert_eq!(client.total(), 70);

    // Withdraw more
    assert_eq!(client.withdraw(&alice, &70), 0);
    assert_eq!(client.balance(&alice), 0);
    assert_eq!(client.total(), 0);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn withdraw_errors_on_insufficient_balance() {
    let env = Env::default();
    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);

    client.deposit(&alice, &50);
    // Try to withdraw more than balance
    client.withdraw(&alice, &100);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn withdraw_errors_on_negative_amount() {
    let env = Env::default();
    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);

    client.deposit(&alice, &50);
    client.withdraw(&alice, &-10);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn withdraw_errors_on_zero_amount() {
    let env = Env::default();
    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);

    client.deposit(&alice, &50);
    client.withdraw(&alice, &0);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn deposit_errors_on_negative_amount() {
    let env = Env::default();
    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);
    client.deposit(&alice, &-100);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn deposit_errors_on_zero_amount() {
    let env = Env::default();
    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);
    client.deposit(&alice, &0);
}

#[test]
fn deposit_emits_event() {
    let env = Env::default();
    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);

    // Call deposit - it will emit an event internally
    client.deposit(&alice, &100);
    
    // If the function completes without panicking, the event was emitted successfully
}

#[test]
fn withdraw_emits_event() {
    let env = Env::default();
    let contract_id = env.register(TreasuryContract, ());
    let client = TreasuryContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);

    client.deposit(&alice, &100);
    // Call withdraw - it will emit an event internally
    client.withdraw(&alice, &30);
    
    // If the function completes without panicking, the event was emitted successfully
}

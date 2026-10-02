#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn fund_and_record_share_track_independently() {
    let env = Env::default();
    let contract_id = env.register(DividendsContract, ());
    let client = DividendsContractClient::new(&env, &contract_id);

    let member = Address::generate(&env);
    let other_member = Address::generate(&env);

    assert_eq!(client.pool(), 0);
    assert_eq!(client.share_of(&member), 0);
    assert_eq!(client.total_shares(), 0);

    assert_eq!(client.fund(&1_000), 1_000);
    assert_eq!(client.fund(&500), 1_500);

    assert_eq!(client.record_share(&member, &300), 300);
    assert_eq!(client.record_share(&member, &200), 500);
    assert_eq!(client.record_share(&other_member, &700), 700);

    assert_eq!(client.pool(), 1_500);
    assert_eq!(client.share_of(&member), 500);
    assert_eq!(client.total_shares(), 1_200);
}

#[test]
fn total_shares_tracks_running_sum_across_members() {
    let env = Env::default();
    let contract_id = env.register(DividendsContract, ());
    let client = DividendsContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);

    assert_eq!(client.total_shares(), 0);

    assert_eq!(client.record_share(&alice, &100), 100);
    assert_eq!(client.total_shares(), 100);
    assert_eq!(client.record_share(&bob, &250), 250);
    assert_eq!(client.total_shares(), 350);
    assert_eq!(client.record_share(&charlie, &75), 75);
    assert_eq!(client.total_shares(), 425);
    assert_eq!(client.record_share(&alice, &50), 150);
    assert_eq!(client.total_shares(), 475);
    assert_eq!(client.record_share(&bob, &25), 275);
    assert_eq!(client.total_shares(), 500);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn fund_rejects_negative_amount() {
    let env = Env::default();
    let contract_id = env.register(DividendsContract, ());
    let client = DividendsContractClient::new(&env, &contract_id);

    client.fund(&-1_000);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn fund_rejects_zero_amount() {
    let env = Env::default();
    let contract_id = env.register(DividendsContract, ());
    let client = DividendsContractClient::new(&env, &contract_id);

    client.fund(&0);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn record_share_rejects_negative_amount() {
    let env = Env::default();
    let contract_id = env.register(DividendsContract, ());
    let client = DividendsContractClient::new(&env, &contract_id);

    let member = Address::generate(&env);
    client.record_share(&member, &-100);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn record_share_rejects_zero_amount() {
    let env = Env::default();
    let contract_id = env.register(DividendsContract, ());
    let client = DividendsContractClient::new(&env, &contract_id);

    let member = Address::generate(&env);
    client.record_share(&member, &0);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn fund_returns_overflow_error_on_i128_max() {
    let env = Env::default();
    let contract_id = env.register(DividendsContract, ());
    let client = DividendsContractClient::new(&env, &contract_id);

    // Fill pool to i128::MAX, then adding 1 must overflow.
    client.fund(&i128::MAX);
    client.fund(&1);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn record_share_returns_overflow_error_on_member_share_max() {
    let env = Env::default();
    let contract_id = env.register(DividendsContract, ());
    let client = DividendsContractClient::new(&env, &contract_id);

    let member = Address::generate(&env);
    // Fill member's share to i128::MAX, then adding 1 must overflow.
    client.record_share(&member, &i128::MAX);
    client.record_share(&member, &1);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn record_share_returns_overflow_error_on_total_shares_max() {
    let env = Env::default();
    let contract_id = env.register(DividendsContract, ());
    let client = DividendsContractClient::new(&env, &contract_id);

    let member_a = Address::generate(&env);
    let member_b = Address::generate(&env);
    // Fill TotalShares to i128::MAX via member_a, then adding 1 via member_b must overflow.
    client.record_share(&member_a, &i128::MAX);
    client.record_share(&member_b, &1);
}

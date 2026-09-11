#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn fund_and_record_share_track_independently() {
    let env = Env::default();
    let contract_id = env.register(DividendsContract, ());
    let client = DividendsContractClient::new(&env, &contract_id);

    let member = Address::generate(&env);

    assert_eq!(client.pool(), 0);
    assert_eq!(client.share_of(&member), 0);

    assert_eq!(client.fund(&1_000), 1_000);
    assert_eq!(client.fund(&500), 1_500);

    assert_eq!(client.record_share(&member, &300), 300);
    assert_eq!(client.record_share(&member, &200), 500);

    assert_eq!(client.pool(), 1_500);
    assert_eq!(client.share_of(&member), 500);
}

#[test]
fn total_shares_tracks_multiple_members_and_updates() {
    let env = Env::default();
    let contract_id = env.register(DividendsContract, ());
    let client = DividendsContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let carol = Address::generate(&env);

    // Starts at zero
    assert_eq!(client.total_shares(), 0);

    // First member
    client.record_share(&alice, &400);
    assert_eq!(client.total_shares(), 400);

    // Second member
    client.record_share(&bob, &200);
    assert_eq!(client.total_shares(), 600);

    // Third member
    client.record_share(&carol, &100);
    assert_eq!(client.total_shares(), 700);

    // Additional allocation to existing member
    client.record_share(&alice, &50);
    assert_eq!(client.total_shares(), 750);

    // Per-member shares are independent of the running total
    assert_eq!(client.share_of(&alice), 450);
    assert_eq!(client.share_of(&bob), 200);
    assert_eq!(client.share_of(&carol), 100);
}

#[test]
fn total_shares_starts_at_zero_when_no_shares_recorded() {
    let env = Env::default();
    let contract_id = env.register(DividendsContract, ());
    let client = DividendsContractClient::new(&env, &contract_id);

    assert_eq!(client.total_shares(), 0);

    // fund does not affect total_shares
    client.fund(&1_000);
    assert_eq!(client.total_shares(), 0);
}

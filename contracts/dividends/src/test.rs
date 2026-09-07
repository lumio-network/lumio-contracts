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

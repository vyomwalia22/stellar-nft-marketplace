use soroban_sdk::{testutils::Address as _, Address, Env, String};
use crate::{NftContractClient, NftContract};

#[test]
fn mint_sets_owner_and_balance_correctly() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let owner = Address::random(&env);
    let contract_id = env.register_contract(None, NftContract);
    let client = NftContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    let token_id = client.mint(&admin, &owner, &String::from_str(&env, "https://example.com/nft/1"));

    assert_eq!(token_id, 1);
    assert_eq!(client.owner_of(1), owner);
    assert_eq!(client.balance_of(&owner), 1);
}

#[test]
fn transfer_moves_ownership_and_updates_balances() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let buyer = Address::random(&env);
    let contract_id = env.register_contract(None, NftContract);
    let client = NftContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    let token_id = client.mint(&admin, &seller, &String::from_str(&env, "https://example.com/nft/2"));
    assert_eq!(client.balance_of(&seller), 1);

    client.transfer(&seller, &buyer, token_id);
    assert_eq!(client.owner_of(token_id), buyer);
    assert_eq!(client.balance_of(&seller), 0);
    assert_eq!(client.balance_of(&buyer), 1);
}

#[test]
fn total_supply_increments_across_mints() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let first = Address::random(&env);
    let second = Address::random(&env);
    let contract_id = env.register_contract(None, NftContract);
    let client = NftContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    client.mint(&admin, &first, &String::from_str(&env, "https://example.com/nft/3"));
    client.mint(&admin, &second, &String::from_str(&env, "https://example.com/nft/4"));

    assert_eq!(NftContract::total_supply(env.clone()), 2);
}

#[test]
#[should_panic]
fn transfer_by_non_owner_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let owner = Address::random(&env);
    let other = Address::random(&env);
    let contract_id = env.register_contract(None, NftContract);
    let client = NftContractClient::new(&env, &contract_id);

    client.initialize(&admin);
    let token_id = client.mint(&admin, &owner, &String::from_str(&env, "https://example.com/nft/5"));
    client.transfer(&other, &Address::random(&env), token_id);
}

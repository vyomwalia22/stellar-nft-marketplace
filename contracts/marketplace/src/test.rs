use soroban_sdk::{testutils::Address as _, Address, Env, IntoVal, String};
use nft_contract::{NftContractClient, NftContract};
use soroban_sdk::token::Client as TokenClient;

#[test]
fn listing_escrows_nft_to_marketplace() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let buyer = Address::random(&env);
    let nft_contract_id = env.register_contract(None, NftContract);
    let marketplace_contract_id = env.register_contract(None, super::MarketplaceContract);
    let payment_token = env.register_stellar_asset_contract_v2();

    let nft_client = NftContractClient::new(&env, &nft_contract_id);
    nft_client.initialize(&admin);
    let token_id = nft_client.mint(&admin, &seller, &String::from_str(&env, "https://example.com/nft/1"));

    let marketplace_client = super::MarketplaceContractClient::new(&env, &marketplace_contract_id);
    marketplace_client.initialize(&admin, 500);
    let listing_id = marketplace_client.list_item(&seller, &nft_contract_id, token_id, &payment_token, &1000i128);

    assert_eq!(nft_client.owner_of(token_id), marketplace_contract_id);
    let listing = marketplace_client.get_listing(listing_id);
    assert!(listing.active);
    assert_eq!(listing.seller, seller);
}

#[test]
fn buying_transfers_nft_to_buyer_and_splits_payment() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let buyer = Address::random(&env);
    let nft_contract_id = env.register_contract(None, NftContract);
    let marketplace_contract_id = env.register_contract(None, super::MarketplaceContract);
    let payment_token = env.register_stellar_asset_contract_v2();

    let nft_client = NftContractClient::new(&env, &nft_contract_id);
    nft_client.initialize(&admin);
    let token_id = nft_client.mint(&admin, &seller, &String::from_str(&env, "https://example.com/nft/2"));

    let mut token = TokenClient::new(&env, &payment_token);
    token.mint(&admin, &buyer, &10000i128);

    let marketplace_client = super::MarketplaceContractClient::new(&env, &marketplace_contract_id);
    marketplace_client.initialize(&admin, 500);
    let listing_id = marketplace_client.list_item(&seller, &nft_contract_id, token_id, &payment_token, &1000i128);
    marketplace_client.buy_item(&buyer, listing_id);

    assert_eq!(nft_client.owner_of(token_id), buyer);
    assert_eq!(token.balance(&buyer), 9000i128);
    assert_eq!(token.balance(&seller), 950i128);
    assert_eq!(token.balance(&admin), 50i128);
}

#[test]
fn cancelling_returns_nft_to_seller() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let nft_contract_id = env.register_contract(None, NftContract);
    let marketplace_contract_id = env.register_contract(None, super::MarketplaceContract);
    let payment_token = env.register_stellar_asset_contract_v2();

    let nft_client = NftContractClient::new(&env, &nft_contract_id);
    nft_client.initialize(&admin);
    let token_id = nft_client.mint(&admin, &seller, &String::from_str(&env, "https://example.com/nft/3"));

    let marketplace_client = super::MarketplaceContractClient::new(&env, &marketplace_contract_id);
    marketplace_client.initialize(&admin, 200);
    let listing_id = marketplace_client.list_item(&seller, &nft_contract_id, token_id, &payment_token, &500i128);
    marketplace_client.cancel_listing(&seller, listing_id);

    assert_eq!(nft_client.owner_of(token_id), seller);
    let listing = marketplace_client.get_listing(listing_id);
    assert!(!listing.active);
}

#[test]
#[should_panic]
fn buying_inactive_listing_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let buyer = Address::random(&env);
    let nft_contract_id = env.register_contract(None, NftContract);
    let marketplace_contract_id = env.register_contract(None, super::MarketplaceContract);
    let payment_token = env.register_stellar_asset_contract_v2();

    let nft_client = NftContractClient::new(&env, &nft_contract_id);
    nft_client.initialize(&admin);
    let token_id = nft_client.mint(&admin, &seller, &String::from_str(&env, "https://example.com/nft/4"));

    let mut token = TokenClient::new(&env, &payment_token);
    token.mint(&admin, &buyer, &1000i128);

    let marketplace_client = super::MarketplaceContractClient::new(&env, &marketplace_contract_id);
    marketplace_client.initialize(&admin, 100);
    let listing_id = marketplace_client.list_item(&seller, &nft_contract_id, token_id, &payment_token, &500i128);
    marketplace_client.cancel_listing(&seller, listing_id);
    marketplace_client.buy_item(&buyer, listing_id);
}

#[test]
#[should_panic]
fn listing_nft_you_do_not_own_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::random(&env);
    let seller = Address::random(&env);
    let impersonator = Address::random(&env);
    let nft_contract_id = env.register_contract(None, NftContract);
    let marketplace_contract_id = env.register_contract(None, super::MarketplaceContract);
    let payment_token = env.register_stellar_asset_contract_v2();

    let nft_client = NftContractClient::new(&env, &nft_contract_id);
    nft_client.initialize(&admin);
    let token_id = nft_client.mint(&admin, &seller, &String::from_str(&env, "https://example.com/nft/5"));

    let marketplace_client = super::MarketplaceContractClient::new(&env, &marketplace_contract_id);
    marketplace_client.initialize(&admin, 100);
    marketplace_client.list_item(&impersonator, &nft_contract_id, token_id, &payment_token, &1000i128);
}

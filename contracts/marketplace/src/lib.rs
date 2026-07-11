#![no_std]

use soroban_sdk::{contracterror, contractimpl, contracttype, symbol, token, Address, Env, IntoVal, String};
use nft_contract::NftContractClient;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    FeeBps,
    ListingCounter,
    Listing(u64),
}

#[contracttype]
#[derive(Clone)]
pub struct Listing {
    pub seller: Address,
    pub nft_contract: Address,
    pub token_id: u32,
    pub payment_token: Address,
    pub price: i128,
    pub active: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct ListedEvent {
    pub listing_id: u64,
    pub seller: Address,
    pub nft_contract: Address,
    pub token_id: u32,
    pub payment_token: Address,
    pub price: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct BoughtEvent {
    pub listing_id: u64,
    pub buyer: Address,
    pub seller: Address,
    pub nft_contract: Address,
    pub token_id: u32,
    pub price: i128,
    pub fee: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct CancelledEvent {
    pub listing_id: u64,
    pub seller: Address,
    pub nft_contract: Address,
    pub token_id: u32,
}

#[contracterror]
pub enum ContractError {
    Unauthorized,
    NotSeller,
    ListingNotActive,
    ListingNotFound,
    SellerDoesNotOwnToken,
    PaymentFailed,
}

pub struct MarketplaceContract;

#[contractimpl]
impl MarketplaceContract {
    pub fn initialize(env: Env, admin: Address, fee_bps: u32) {
        if env.storage().has(&DataKey::Admin) {
            panic!(ContractError::Unauthorized);
        }
        env.storage().set(&DataKey::Admin, &admin);
        env.storage().set(&DataKey::FeeBps, &fee_bps);
        env.storage().set(&DataKey::ListingCounter, &0u64);
    }

    pub fn list_item(
        env: Env,
        seller: Address,
        nft_contract: Address,
        token_id: u32,
        payment_token: Address,
        price: i128,
    ) -> u64 {
        seller.require_auth();
        let nft_client = NftContractClient::new(&env, &nft_contract);
        let owner = nft_client.owner_of(token_id);
        if owner != seller {
            panic!(ContractError::SellerDoesNotOwnToken);
        }

        nft_client.transfer(&seller, &env.current_contract_address(), token_id);

        let listing_id: u64 = env.storage().get(&DataKey::ListingCounter).unwrap_or(0u64) + 1;
        let listing = Listing {
            seller: seller.clone(),
            nft_contract: nft_contract.clone(),
            token_id,
            payment_token: payment_token.clone(),
            price,
            active: true,
        };
        env.storage().set(&DataKey::Listing(listing_id), &listing);
        env.storage().set(&DataKey::ListingCounter, &listing_id);

        env.events().publish((symbol!("listed"),), ListedEvent {
            listing_id,
            seller,
            nft_contract,
            token_id,
            payment_token,
            price,
        });

        listing_id
    }

    pub fn buy_item(env: Env, buyer: Address, listing_id: u64) {
        buyer.require_auth();
        let mut listing: Listing = env.storage().get(&DataKey::Listing(listing_id)).unwrap().unwrap_or_else(|| panic!(ContractError::ListingNotFound));
        if !listing.active {
            panic!(ContractError::ListingNotActive);
        }

        let fee_bps: u32 = env.storage().get(&DataKey::FeeBps).unwrap_or(0u32);
        let fee = listing.price * (fee_bps as i128) / 10000;
        let seller_amount = listing.price - fee;
        let admin: Address = env.storage().get(&DataKey::Admin).unwrap().unwrap();

        let token_client = token::Client::new(&env, &listing.payment_token);
        token_client.transfer(&buyer, &listing.seller, &seller_amount);
        if fee > 0 {
            token_client.transfer(&buyer, &admin, &fee);
        }

        let nft_client = NftContractClient::new(&env, &listing.nft_contract);
        nft_client.transfer(&env.current_contract_address(), &buyer, listing.token_id);

        listing.active = false;
        env.storage().set(&DataKey::Listing(listing_id), &listing);

        env.events().publish((symbol!("bought"),), BoughtEvent {
            listing_id,
            buyer,
            seller: listing.seller,
            nft_contract: listing.nft_contract,
            token_id: listing.token_id,
            price: listing.price,
            fee,
        });
    }

    pub fn cancel_listing(env: Env, seller: Address, listing_id: u64) {
        seller.require_auth();
        let mut listing: Listing = env.storage().get(&DataKey::Listing(listing_id)).unwrap().unwrap_or_else(|| panic!(ContractError::ListingNotFound));
        if !listing.active {
            panic!(ContractError::ListingNotActive);
        }
        if listing.seller != seller {
            panic!(ContractError::NotSeller);
        }

        let nft_client = NftContractClient::new(&env, &listing.nft_contract);
        nft_client.transfer(&env.current_contract_address(), &seller, listing.token_id);

        listing.active = false;
        env.storage().set(&DataKey::Listing(listing_id), &listing);

        env.events().publish((symbol!("cancelled"),), CancelledEvent {
            listing_id,
            seller,
            nft_contract: listing.nft_contract,
            token_id: listing.token_id,
        });
    }

    pub fn get_listing(env: Env, listing_id: u64) -> Listing {
        env.storage().get(&DataKey::Listing(listing_id)).unwrap().unwrap_or_else(|| panic!(ContractError::ListingNotFound))
    }

    pub fn total_listings(env: Env) -> u64 {
        env.storage().get(&DataKey::ListingCounter).unwrap_or(0u64)
    }
}

pub struct MarketplaceContractClient<'a> {
    env: &'a Env,
    contract_id: Address,
}

impl<'a> MarketplaceContractClient<'a> {
    pub fn new(env: &'a Env, contract_id: &Address) -> Self {
        Self { env, contract_id: contract_id.clone() }
    }

    pub fn initialize(&self, admin: &Address, fee_bps: u32) {
        self.env.invoke_contract(&self.contract_id, &SorobanSymbol::short("initialize"), (admin, &fee_bps));
    }

    pub fn list_item(&self, seller: &Address, nft_contract: &Address, token_id: u32, payment_token: &Address, price: &i128) -> u64 {
        self.env
            .invoke_contract(
                &self.contract_id,
                &SorobanSymbol::short("list_item"),
                (seller, nft_contract, token_id, payment_token, price),
            )
            .try_into_val(self.env)
    }

    pub fn buy_item(&self, buyer: &Address, listing_id: u64) {
        self.env
            .invoke_contract(&self.contract_id, &SorobanSymbol::short("buy_item"), (buyer, &listing_id));
    }

    pub fn cancel_listing(&self, seller: &Address, listing_id: u64) {
        self.env
            .invoke_contract(&self.contract_id, &SorobanSymbol::short("cancel_listing"), (seller, &listing_id));
    }

    pub fn get_listing(&self, listing_id: u64) -> Listing {
        self.env
            .invoke_contract(&self.contract_id, &SorobanSymbol::short("get_listing"), (&listing_id,))
            .try_into_val(self.env)
    }
}

mod test;

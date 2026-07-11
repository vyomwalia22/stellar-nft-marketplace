#![no_std]

use soroban_sdk::{contracterror, contractimpl, contracttype, symbol, Address, Env, String, Symbol as SorobanSymbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Counter,
    TotalSupply,
    Owner(u32),
    TokenUri(u32),
    Balance(Address),
}

#[contracttype]
#[derive(Clone)]
pub struct MintEvent {
    pub owner: Address,
    pub token_id: u32,
    pub uri: String,
}

#[contracttype]
#[derive(Clone)]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    pub token_id: u32,
}

#[contracterror]
pub enum ContractError {
    Unauthorized,
    TokenNotFound,
    NotOwner,
    AlreadyInitialized,
}

pub struct NftContract;

#[contractimpl]
impl NftContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().has(&DataKey::Admin) {
            panic!(ContractError::AlreadyInitialized);
        }
        env.storage().set(&DataKey::Admin, &admin);
        env.storage().set(&DataKey::Counter, &0u32);
        env.storage().set(&DataKey::TotalSupply, &0u32);
    }

    pub fn mint(env: Env, to: Address, uri: String) -> u32 {
        let admin = env.storage().get(&DataKey::Admin).unwrap().unwrap();
        admin.require_auth();
        let token_id: u32 = env.storage().get(&DataKey::Counter).unwrap_or(0u32);
        let next_id = token_id + 1;
        env.storage().set(&DataKey::Counter, &next_id);
        env.storage().set(&DataKey::Owner(next_id), &to);
        env.storage().set(&DataKey::TokenUri(next_id), &uri);

        let balance_key = DataKey::Balance(to.clone());
        let owner_balance: u32 = env.storage().get(&balance_key).unwrap_or(0u32);
        env.storage().set(&balance_key, &(owner_balance + 1));

        let total_supply: u32 = env.storage().get(&DataKey::TotalSupply).unwrap_or(0u32);
        env.storage().set(&DataKey::TotalSupply, &(total_supply + 1));

        env.events().publish((symbol!("mint"),), MintEvent { owner: to.clone(), token_id: next_id, uri });
        next_id
    }

    pub fn owner_of(env: Env, token_id: u32) -> Address {
        env.storage()
            .get(&DataKey::Owner(token_id))
            .unwrap()
            .unwrap_or_else(|| panic!(ContractError::TokenNotFound))
    }

    pub fn token_uri(env: Env, token_id: u32) -> String {
        env.storage()
            .get(&DataKey::TokenUri(token_id))
            .unwrap()
            .unwrap_or_else(|| panic!(ContractError::TokenNotFound))
    }

    pub fn balance_of(env: Env, owner: Address) -> u32 {
        env.storage().get(&DataKey::Balance(owner)).unwrap_or(0u32)
    }

    pub fn transfer(env: Env, from: Address, to: Address, token_id: u32) {
        from.require_auth();
        let current_owner = env.storage().get(&DataKey::Owner(token_id)).unwrap().unwrap_or_else(|| panic!(ContractError::TokenNotFound));
        if current_owner != from {
            panic!(ContractError::NotOwner);
        }

        env.storage().set(&DataKey::Owner(token_id), &to.clone());

        let from_balance_key = DataKey::Balance(from.clone());
        let from_balance: u32 = env.storage().get(&from_balance_key).unwrap_or(0u32);
        env.storage().set(&from_balance_key, &(from_balance - 1));

        let to_balance_key = DataKey::Balance(to.clone());
        let to_balance: u32 = env.storage().get(&to_balance_key).unwrap_or(0u32);
        env.storage().set(&to_balance_key, &(to_balance + 1));

        env.events().publish((symbol!("transfer"),), TransferEvent { from, to, token_id });
    }

    pub fn total_supply(env: Env) -> u32 {
        env.storage().get(&DataKey::TotalSupply).unwrap_or(0u32)
    }
}

pub struct NftContractClient<'a> {
    env: &'a Env,
    contract_id: Address,
}

impl<'a> NftContractClient<'a> {
    pub fn new(env: &'a Env, contract_id: &Address) -> Self {
        Self { env, contract_id: contract_id.clone() }
    }

    pub fn initialize(&self, admin: &Address) {
        self.env.invoke_contract(&self.contract_id, &SorobanSymbol::short("initialize"), (admin,));
    }

    pub fn mint(&self, admin: &Address, to: &Address, uri: &String) -> u32 {
        self.env
            .invoke_contract(&self.contract_id, &SorobanSymbol::short("mint"), (admin, to, uri))
            .try_into_val(self.env)
    }

    pub fn owner_of(&self, token_id: u32) -> Address {
        self.env
            .invoke_contract(&self.contract_id, &SorobanSymbol::short("owner_of"), (token_id,))
            .try_into_val(self.env)
    }

    pub fn token_uri(&self, token_id: u32) -> String {
        self.env
            .invoke_contract(&self.contract_id, &SorobanSymbol::short("token_uri"), (token_id,))
            .try_into_val(self.env)
    }

    pub fn balance_of(&self, owner: &Address) -> u32 {
        self.env
            .invoke_contract(&self.contract_id, &SorobanSymbol::short("balance_of"), (owner,))
            .try_into_val(self.env)
    }

    pub fn transfer(&self, from: &Address, to: &Address, token_id: u32) {
        self.env.invoke_contract(&self.contract_id, &SorobanSymbol::short("transfer"), (from, to, token_id));
    }
}

mod test;

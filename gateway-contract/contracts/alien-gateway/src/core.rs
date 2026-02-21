use soroban_sdk::{
    contracttype, symbol_short, Address, Env, Symbol, BytesN,
};

// Storage Keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Username,
    Owner,
    CreatedAt,
}

// Event
const INIT_EVENT: Symbol = symbol_short!("INIT");

pub struct CoreContract;

impl CoreContract {
    pub fn init(env: Env, username: Symbol) {
        // Prevent re-init
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("Contract already initialized");
        }

        // Validate username
        let name = username.to_string();

        if name.is_empty() {
            panic!("Username cannot be empty");
        }

        if name.len() > 32 {
            panic!("Username too long");
        }

        let owner = env.invoker();

        // Store values
        env.storage().instance().set(&DataKey::Username, &username);
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage()
            .instance()
            .set(&DataKey::CreatedAt, &env.ledger().timestamp());

        // Emit event
        env.events().publish(
            (INIT_EVENT,),
            (username, owner)
        );
    }

    // Getters
    pub fn get_username(env: Env) -> Symbol {
        env.storage()
            .instance()
            .get(&DataKey::Username)
            .unwrap()
    }

    pub fn get_owner(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Owner)
            .unwrap()
    }

    pub fn get_created_at(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::CreatedAt)
            .unwrap()
    }
}

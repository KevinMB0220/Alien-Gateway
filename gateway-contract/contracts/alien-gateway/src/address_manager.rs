use soroban_sdk::{contracttype, symbol_short, Address, Env, Symbol};

// Storage Keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owner,
    Address(Address),
    MasterAddress,
}

// Event Symbol
const MASTER_SET: Symbol = symbol_short!("MASTER_SET");

pub struct AddressManager;

impl AddressManager {
    // Initialize contract with owner
    pub fn init(env: Env, owner: Address) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Owner, &owner);
    }

    // Helper: check owner
    fn require_owner(env: &Env) {
        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::Owner)
            .unwrap();

        owner.require_auth();
    }

    // Helper: check address exists
    fn address_exists(env: &Env, address: &Address) -> bool {
        env.storage()
            .instance()
            .has(&DataKey::Address(address.clone()))
    }

    // Optional helper to register address
    pub fn register_address(env: Env, address: Address) {
        Self::require_owner(&env);
        env.storage()
            .instance()
            .set(&DataKey::Address(address.clone()), &true);
    }

    // ✅ Main Function
    pub fn set_master_stellar_address(env: Env, address: Address) {
        Self::require_owner(&env);

        // Address must exist
        if !Self::address_exists(&env, &address) {
            panic!("Address does not exist");
        }

        // Unset previous master (if any)
        if env.storage().instance().has(&DataKey::MasterAddress) {
            env.storage().instance().remove(&DataKey::MasterAddress);
        }

        // Set new master
        env.storage()
            .instance()
            .set(&DataKey::MasterAddress, &address);

        // Emit Event
        env.events().publish(
            (MASTER_SET,),
            address
        );
    }

    // Getter
    pub fn get_master(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::MasterAddress)
    }
}

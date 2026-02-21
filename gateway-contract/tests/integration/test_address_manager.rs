use soroban_sdk::{testutils::{Address as _}, Address, Env};
use alien_gateway::AddressManager;

#[test]
fn test_master_assignment() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let user = Address::generate(&env);

    AddressManager::init(env.clone(), owner.clone());

    owner.require_auth();

    AddressManager::register_address(env.clone(), user.clone());
    AddressManager::set_master_stellar_address(env.clone(), user.clone());

    let master = AddressManager::get_master(env.clone()).unwrap();
    assert_eq!(master, user);
}

#[test]
fn test_switch_master() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    AddressManager::init(env.clone(), owner.clone());

    owner.require_auth();
    AddressManager::register_address(env.clone(), user1.clone());
    AddressManager::register_address(env.clone(), user2.clone());

    AddressManager::set_master_stellar_address(env.clone(), user1.clone());
    AddressManager::set_master_stellar_address(env.clone(), user2.clone());

    let master = AddressManager::get_master(env.clone()).unwrap();
    assert_eq!(master, user2);
}

#[test]
#[should_panic(expected = "Address does not exist")]
fn test_non_existent_address_fails() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let user = Address::generate(&env);

    AddressManager::init(env.clone(), owner.clone());

    owner.require_auth();
    AddressManager::set_master_stellar_address(env.clone(), user.clone());
}

#[test]
#[should_panic]
fn test_unauthorized_fails() {
    let env = Env::default();
    let owner = Address::generate(&env);
    let user = Address::generate(&env);
    let attacker = Address::generate(&env);

    AddressManager::init(env.clone(), owner.clone());

    owner.require_auth();
    AddressManager::register_address(env.clone(), user.clone());

    attacker.require_auth();
    AddressManager::set_master_stellar_address(env.clone(), user.clone());
}

use soroban_sdk::Env;

use crate::{Contract, ContractClient};

#[test]
fn increment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    assert_eq!(client.increase(), 1);
    assert_eq!(client.increase(), 2);
    assert_eq!(client.increase(), 3);
}

#[test]
fn increment_by() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    assert_eq!(client.increase_by(&2), 2);
    assert_eq!(client.increase_by(&3), 5);
}

#[test]
fn decrement() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    assert_eq!(client.increase(), 1);
    assert_eq!(client.increase(), 2);
    assert_eq!(client.increase(), 3);
    assert_eq!(client.decrease(), 2);
    assert_eq!(client.decrease(), 1);
}

#[test]
fn decrement_by() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    assert_eq!(client.increase_by(&10), 10);
    assert_eq!(client.decrease_by(&5), 5);
    assert_eq!(client.decrease_by(&4), 1);
}

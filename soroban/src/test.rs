use soroban_sdk::{
    symbol_short,
    testutils::Logs,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Events},
    vec, Address, Env, IntoVal, Symbol,
};

use crate::{Contract, ContractClient, Error};

extern crate std;

#[test]
fn increase_by_address() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let user_1 = Address::random(&env);
    let user_2 = Address::random(&env);

    assert_eq!(client.increase_by_address(&user_1, &5), 5);

    // Verify that the user indeed had to authorize a call of `increment` with
    // the expected arguments:
    assert_eq!(
        env.auths(),
        std::vec![(
            // Address for which authorization check is performed
            user_1.clone(),
            // Invocation tree that needs to be authorized
            AuthorizedInvocation {
                // Function that is authorized. Can be a contract function or
                // a host function that requires authorization.
                function: AuthorizedFunction::Contract((
                    // Address of the called contract
                    contract_id.clone(),
                    // Name of the called function
                    Symbol::new(&env, "increase_by_address"),
                    // Arguments used to call `increment` (converted to the env-managed vector via `into_val`)
                    (user_1.clone(), 5_u32).into_val(&env),
                )),
                // The contract doesn't call any other contracts that require
                // authorization,
                sub_invocations: std::vec![]
            }
        )]
    );

    // Do more `increment` calls. It's not necessary to verify authorizations
    // for every one of them as we don't expect the auth logic to change from
    // call to call.
    assert_eq!(client.increase_by_address(&user_1, &2), 7);
    assert_eq!(client.increase_by_address(&user_2, &1), 1);
    assert_eq!(client.increase_by_address(&user_1, &3), 10);
    assert_eq!(client.increase_by_address(&user_2, &4), 5);
}

#[test]
fn increase_by_person() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    client.increase_by_person(&symbol_short!("Frank"));
    client.increase_by_person(&symbol_short!("Frank"));

    let logs = env.logs().all();
    std::print!("{}", logs.join("\n"));
}

#[test]
fn constrained_increase() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    assert_eq!(client.try_constrained_increase(), Ok(Ok(1)));
    assert_eq!(
        client.try_constrained_increase(),
        Err(Ok(Error::LimitReached))
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn constrained_increase_w_panic() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    assert_eq!(client.constrained_increase(), 1);
    client.constrained_increase();
}

#[test]
fn increment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    assert_eq!(client.increase(), 1);
    assert_eq!(client.increase(), 2);
    assert_eq!(client.increase(), 3);

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("increase")).into_val(&env),
                1u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("increase")).into_val(&env),
                2u32.into_val(&env)
            ),
            (
                contract_id,
                (symbol_short!("COUNTER"), symbol_short!("increase")).into_val(&env),
                3u32.into_val(&env)
            ),
        ]
    );
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

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("increase")).into_val(&env),
                1u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("increase")).into_val(&env),
                2u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("increase")).into_val(&env),
                3u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("decrease")).into_val(&env),
                2u32.into_val(&env)
            ),
            (
                contract_id,
                (symbol_short!("COUNTER"), symbol_short!("decrease")).into_val(&env),
                1u32.into_val(&env)
            ),
        ]
    );
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

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, vec, Address, Env, Symbol,
    Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    LimitReached = 1,
}

pub const COUNTER: Symbol = symbol_short!("COUNTER");
pub const PEOPLE: Symbol = symbol_short!("PERSON");

#[contracttype]
pub enum DataKey {
    COUNTER(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Person {
    pub name: Symbol,
    pub count: u32,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }

    pub fn increase_by_person(env: Env, name: Symbol) -> Vec<Person> {
        let mut people: Vec<Person> = env
            .storage()
            .instance()
            .get(&PEOPLE)
            .unwrap_or(Vec::new(&env));

        match people.iter().position(|p| p.name.eq(&name)) {
            Some(idx) => {
                let person = people.get(idx as u32).unwrap();
                people.set(
                    idx as u32,
                    Person {
                        name: person.name.clone(),
                        count: person.count + 1,
                    },
                );
            }
            None => people.push_back(Person {
                name: name.clone(),
                count: 1,
            }),
        }

        env.storage().instance().set(&PEOPLE, &people.clone());

        people
    }

    pub fn increase(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count = count.saturating_add(1);
        env.events()
            .publish((COUNTER, symbol_short!("increase")), count);
        env.storage().instance().set(&COUNTER, &count);
        count
    }

    pub fn increase_by_address(env: Env, address: Address, value: u32) -> u32 {
        address.require_auth();

        let key = DataKey::COUNTER(address.clone());

        let mut count: u32 = env.storage().instance().get(&key).unwrap_or_default();

        count = count.saturating_add(value);

        env.storage().instance().set(&key, &count);

        count
    }

    pub fn constrained_increase(env: Env) -> Result<u32, Error> {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count = count.saturating_add(1);

        if count < 2 {
            env.storage().instance().set(&COUNTER, &count);
            Ok(count)
        } else {
            Err(Error::LimitReached)
        }
    }

    pub fn increase_by(env: Env, by: u32) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count = count.saturating_add(by);
        env.storage().instance().set(&COUNTER, &count);
        count
    }

    pub fn decrease(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count = count.saturating_sub(1);
        env.events()
            .publish((COUNTER, symbol_short!("decrease")), count);
        env.storage().instance().set(&COUNTER, &count);
        count
    }

    pub fn decrease_by(env: Env, by: u32) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count = count.saturating_sub(by);
        env.storage().instance().set(&COUNTER, &count);
        count
    }
}

#[cfg(test)]
mod test;

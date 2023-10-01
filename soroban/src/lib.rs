#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, vec, Env, Symbol, Vec};

pub const COUNTER: Symbol = symbol_short!("COUNTER");
pub const PEOPLE: Symbol = symbol_short!("PERSON");

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

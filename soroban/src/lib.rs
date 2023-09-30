#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

pub const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }

    pub fn increase(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count = count.saturating_add(1);
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

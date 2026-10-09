#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
        pub fn bump(env: Env, from: Address) -> u32 {
            from.require_auth();
            let n: u32 = env.storage().instance().get(&symbol_short!("n")).unwrap_or(0) + 1;
            env.storage().instance().set(&symbol_short!("n"), &n);
            env.events().publish((symbol_short!("bump"), from), n);
            n
        }
}

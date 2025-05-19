// src/balance.rs

use soroban_sdk::{Env, Address};
use crate::storage_types::BALANCES_KEY;

pub fn balance_of(e: Env, owner: Address) -> i128 {
    let balances = e.storage().instance().get::<_, soroban_sdk::Map<Address, i128>>(&BALANCES_KEY)
        .unwrap_or(soroban_sdk::Map::new(&e));
    balances.get(owner).unwrap_or(0)
}

pub fn set_balance(e: Env, owner: Address, balance: i128) {
    let mut balances = e.storage().instance().get::<_, soroban_sdk::Map<Address, i128>>(&BALANCES_KEY)
        .unwrap_or(soroban_sdk::Map::new(&e));
    if balance == 0 {
        balances.remove(owner);
    } else {
        balances.set(owner, balance);
    }
    e.storage().instance().set(&BALANCES_KEY, &balances);
}
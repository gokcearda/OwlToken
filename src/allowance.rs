// src/allowance.rs

use soroban_sdk::{Env, Address, Map};
use crate::storage_types::ALLOWANCES_KEY;
use crate::balance;

pub fn approve(e: Env, owner: Address, spender: Address, amount: i128) {
    let mut allowances = e
        .storage()
        .instance()
        .get::<_, Map<(Address, Address), i128>>(&ALLOWANCES_KEY)
        .unwrap_or(Map::new(&e));
    allowances.set((owner, spender), amount);
    e.storage().instance().set(&ALLOWANCES_KEY, &allowances);
}

pub fn allowance(e: Env, owner: Address, spender: Address) -> i128 {
    let allowances = e
        .storage()
        .instance()
        .get::<_, Map<(Address, Address), i128>>(&ALLOWANCES_KEY)
        .unwrap_or(Map::new(&e));
    allowances.get((owner, spender)).unwrap_or(0)
}

pub fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
    let mut allowances = e
        .storage()
        .instance()
        .get::<_, Map<(Address, Address), i128>>(&ALLOWANCES_KEY)
        .unwrap_or(Map::new(&e));

    let mut allowance_val = allowances.get((from.clone(), spender.clone())).unwrap_or(0);
    if allowance_val < amount {
        panic!("Allowance exceeded");
    }
    allowance_val -= amount;
    allowances.set((from.clone(), spender.clone()), allowance_val);
    e.storage().instance().set(&ALLOWANCES_KEY, &allowances);

    let from_balance = balance::balance_of(e.clone(), from.clone());
    if from_balance < amount {
        panic!("Insufficient balance");
    }
    balance::set_balance(e.clone(), from.clone(), from_balance - amount);

    let to_balance = balance::balance_of(e.clone(), to.clone());
    balance::set_balance(e, to, to_balance + amount);
}
// src/contract.rs

use soroban_sdk::{Address, Env};

use crate::{
    admin,
    balance,
    freeze,
    storage_types::{TOTAL_SUPPLY_KEY},
};

pub fn total_supply(e: Env) -> i128 {
    e.storage().instance().get(&TOTAL_SUPPLY_KEY).unwrap_or(0)
}

pub fn transfer(e: Env, from: Address, to: Address, amount: i128) {
    if freeze::is_frozen(e.clone(), from.clone()) {
        panic!("Sender account is frozen");
    }
    if freeze::is_frozen(e.clone(), to.clone()) {
        panic!("Recipient account is frozen");
    }
    if amount <= 0 {
        panic!("Amount must be positive");
    }
    let from_balance = balance::balance_of(e.clone(), from.clone());
    if from_balance < amount {
        panic!("Insufficient balance");
    }

    balance::set_balance(e.clone(), from.clone(), from_balance - amount);

    let to_balance = balance::balance_of(e.clone(), to.clone());
    balance::set_balance(e, to, to_balance + amount);
}
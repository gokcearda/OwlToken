// src/admin.rs

use soroban_sdk::{Env, Address};
use crate::storage_types::{ADMIN_KEY, TOTAL_SUPPLY_KEY};
use crate::balance;

fn require_admin(e: &Env, caller: &Address) {
    let admin = get_admin(e.clone());
    if &admin != caller {
        panic!("Caller is not admin");
    }
}

pub fn set_admin(e: Env, caller: Address, new_admin: Address) {
    require_admin(&e, &caller);
    e.storage().instance().set(&ADMIN_KEY, &new_admin);
}

pub fn get_admin(e: Env) -> Address {
    e.storage().instance().get(&ADMIN_KEY).expect("Admin not set")
}

pub fn mint(e: Env, admin_addr: Address, to: Address, amount: i128) {
    require_admin(&e, &admin_addr);
    if amount <= 0 { panic!("Mint amount should be positive"); }

    let mut supply = e.storage().instance().get::<_, i128>(&TOTAL_SUPPLY_KEY).unwrap_or(0);
    supply += amount;
    e.storage().instance().set(&TOTAL_SUPPLY_KEY, &supply);

    let current_balance = balance::balance_of(e.clone(), to.clone());
    balance::set_balance(e, to, current_balance + amount);
}

pub fn burn(e: Env, admin_addr: Address, from: Address, amount: i128) {
    require_admin(&e, &admin_addr);
    if amount <= 0 { panic!("Burn amount should be positive"); }

    let current_balance = balance::balance_of(e.clone(), from.clone());
    if current_balance < amount {
        panic!("Not enough balance to burn");
    }
    balance::set_balance(e.clone(), from.clone(), current_balance - amount);

    let mut supply = e.storage().instance().get::<_, i128>(&TOTAL_SUPPLY_KEY).unwrap_or(0);
    supply -= amount;
    if supply < 0 { panic!("Supply cannot be negative"); }
    e.storage().instance().set(&TOTAL_SUPPLY_KEY, &supply);
}
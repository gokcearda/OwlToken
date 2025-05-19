// src/freeze.rs

use soroban_sdk::{Env, Address};
use crate::storage_types::FREEZE_KEY;

pub fn freeze_account(e: Env, admin: Address, target: Address) {
    let admin_addr = crate::admin::get_admin(e.clone());
    if admin != admin_addr {
        panic!("Only admin can freeze accounts");
    }
    let mut freeze_map = e
        .storage()
        .instance()
        .get::<_, soroban_sdk::Map<Address, bool>>(&FREEZE_KEY)
        .unwrap_or(soroban_sdk::Map::new(&e));
    freeze_map.set(target, true);
    e.storage().instance().set(&FREEZE_KEY, &freeze_map);
}

pub fn unfreeze_account(e: Env, admin: Address, target: Address) {
    let admin_addr = crate::admin::get_admin(e.clone());
    if admin != admin_addr {
        panic!("Only admin can unfreeze accounts");
    }
    let mut freeze_map = e
        .storage()
        .instance()
        .get::<_, soroban_sdk::Map<Address, bool>>(&FREEZE_KEY)
        .unwrap_or(soroban_sdk::Map::new(&e));
    freeze_map.set(target, false);
    e.storage().instance().set(&FREEZE_KEY, &freeze_map);
}

pub fn is_frozen(e: Env, target: Address) -> bool {
    let freeze_map = e
        .storage()
        .instance()
        .get::<_, soroban_sdk::Map<Address, bool>>(&FREEZE_KEY)
        .unwrap_or(soroban_sdk::Map::new(&e));
    freeze_map.get(target).unwrap_or(false)
}
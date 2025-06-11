// src/lib.rs

#![no_std]

mod contract;
mod admin;
mod balance;
mod allowance;
mod metadata;
mod storage_types; 
mod freeze;

use soroban_sdk::{
    contract, contractimpl, Address, Env, Symbol,
};

pub use contract::*;
pub use admin::*;
pub use balance::*;
pub use allowance::*;
pub use metadata::*;
pub use storage_types::*;
pub use freeze::*;

#[contract]
pub struct OwlToken;

#[contractimpl]
impl OwlToken {
    pub fn initialize(e: Env, admin: Address) {
        if e.storage().instance().has(&ADMIN_KEY) {
            panic!("Contract already initialized");
        }
        e.storage().instance().set(&ADMIN_KEY, &admin);
    }

    pub fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        contract::transfer(e, from, to, amount)
    }

    pub fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        allowance::transfer_from(e, spender, from, to, amount)
    }

    pub fn balance_of(e: Env, owner: Address) -> i128 {
        balance::balance_of(e, owner)
    }

    pub fn total_supply(e: Env) -> i128 {
        contract::total_supply(e)
    }

    pub fn approve(e: Env, owner: Address, spender: Address, amount: i128) {
        allowance::approve(e, owner, spender, amount)
    }

    pub fn allowance(e: Env, owner: Address, spender: Address) -> i128 {
        allowance::allowance(e, owner, spender)
    }

    pub fn freeze_account(e: Env, admin: Address, target: Address) {
        freeze::freeze_account(e, admin, target)
    }

    pub fn unfreeze_account(e: Env, admin: Address, target: Address) {
        freeze::unfreeze_account(e, admin, target)
    }

    pub fn is_frozen(e: Env, target: Address) -> bool {
        freeze::is_frozen(e, target)
    }

    pub fn set_admin(e: Env, caller: Address, new_admin: Address) {
        admin::set_admin(e, caller, new_admin)
    }

    pub fn get_admin(e: Env) -> Address {
        admin::get_admin(e)
    }

    pub fn mint(e: Env, admin: Address, to: Address, amount: i128) {
        admin::mint(e, admin, to, amount)
    }

    pub fn burn(e: Env, admin: Address, from: Address, amount: i128) {
        admin::burn(e, admin, from, amount)
    }

    pub fn name(_e: Env) -> Symbol {
        metadata::name()
    }

    pub fn symbol(_e: Env) -> Symbol {
        metadata::symbol()
    }

    pub fn decimals(_e: Env) -> u32 {
        metadata::decimals()
    }
}
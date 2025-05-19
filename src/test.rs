// src/test.rs

#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env, Address};

#[test]
fn test_admin_and_mint_and_transfer() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let user = Address::generate(&e);

    // Admin'i ata
    admin::set_admin(e.clone(), admin.clone(), admin.clone());
    assert_eq!(admin::get_admin(e.clone()), admin);

    // Mint
    admin::mint(e.clone(), admin.clone(), user.clone(), 1000);
    assert_eq!(balance::balance_of(e.clone(), user.clone()), 1000);
    assert_eq!(contract::total_supply(e.clone()), 1000);

    // Transfer
    let user2 = Address::generate(&e);
    contract::transfer(e.clone(), user.clone(), user2.clone(), 400);
    assert_eq!(balance::balance_of(e.clone(), user.clone()), 600);
    assert_eq!(balance::balance_of(e.clone(), user2.clone()), 400);
}

#[test]
#[should_panic(expected = "Sender account is frozen")]
fn test_freeze_and_prevent_transfer() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);

    // Admin ve mint
    admin::set_admin(e.clone(), admin.clone(), admin.clone());
    admin::mint(e.clone(), admin.clone(), user1.clone(), 500);
    assert_eq!(balance::balance_of(e.clone(), user1.clone()), 500);

    // Freeze
    freeze::freeze_account(e.clone(), admin.clone(), user1.clone());
    assert!(freeze::is_frozen(e.clone(), user1.clone()));

    // Transfer yapmaya çalışınca hata fırlatmalı
    contract::transfer(e, user1, user2, 100);
}

#[test]
fn test_unfreeze_allows_transfer() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);

    admin::set_admin(e.clone(), admin.clone(), admin.clone());
    admin::mint(e.clone(), admin.clone(), user1.clone(), 300);

    freeze::freeze_account(e.clone(), admin.clone(), user1.clone());
    freeze::unfreeze_account(e.clone(), admin.clone(), user1.clone());

    contract::transfer(e.clone(), user1.clone(), user2.clone(), 100);
    assert_eq!(balance::balance_of(e.clone(), user1.clone()), 200);
    assert_eq!(balance::balance_of(e, user2.clone()), 100);
}

#[test]
fn test_approve_and_transfer_from() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let owner = Address::generate(&e);
    let spender = Address::generate(&e);
    let recipient = Address::generate(&e);

    admin::set_admin(e.clone(), admin.clone(), admin.clone());
    admin::mint(e.clone(), admin.clone(), owner.clone(), 1000);

    // owner onay veriyor
    allowance::approve(e.clone(), owner.clone(), spender.clone(), 500);
    assert_eq!(allowance::allowance(e.clone(), owner.clone(), spender.clone()), 500);

    // spender, owner'ın bakiyesinden transfer yapıyor
    allowance::transfer_from(e.clone(), spender.clone(), owner.clone(), recipient.clone(), 200);
    assert_eq!(balance::balance_of(e.clone(), owner.clone()), 800);
    assert_eq!(balance::balance_of(e.clone(), recipient.clone()), 200);
    assert_eq!(allowance::allowance(e, owner.clone(), spender.clone()), 300);
}
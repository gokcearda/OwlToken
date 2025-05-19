// src/metadata.rs

use soroban_sdk::Symbol;

pub fn name() -> Symbol {
    Symbol::short("OwlToken")
}

pub fn symbol() -> Symbol {
    Symbol::short("OWL")
}

pub fn decimals() -> u32 {
    8
}
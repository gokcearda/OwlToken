// src/metadata.rs

use soroban_sdk::{Symbol, symbol_short};

pub fn name() -> Symbol {
    symbol_short!("OwlToken")
}

pub fn symbol() -> Symbol {
    symbol_short!("OWL")  
}

pub fn decimals() -> u32 {
    8
}
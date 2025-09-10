/*
Section work will be here , wills start with w1.rs and so on also they called from mod.rs
- Simiulates a decsk of playing cards
- This will have the code for the deck
*/

#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};
use rand::{rng, seq::SliceRandom};
use yansi::Paint;

// --- Main Function ---
pub fn s2_main() {
    greet();
}

// --- Sub Functions---

// Greet function

fn greet() {
    pswg("Section 2 - Bank  ".to_string());
    make_bank();
}

/*
Buiilding the bank struct and accounts
Step 1 - is defining the types which are being done below
*/

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

// the acccounts element in the struct below as a Vector of Account structs which has been defined above
#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

// Inherent implementation of the bank
impl Bank {
    // new acccount information
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

// Function to make a new bank

fn make_bank() {
    header("Making a new Bank");

    let bank = Bank::new();
    println!("{}", bank)
}

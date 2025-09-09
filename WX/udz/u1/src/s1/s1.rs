/*
Section work will be here , wills start with w1.rs and so on also they called from mod.rs
- Simiulates a decsk of playing cards
- This will have the code for the deck
*/

#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};

// --- Main Function ---
pub fn s1_main() {
    greet();
    func1();
}

// --- Sub Functions---

// Greet function

fn greet() {
    pswg("Section 2 - Cards  ".to_string())
}

struct Deck {
    cards: Vec<String>,
}

fn func1() {
    header("Deck Function")
}

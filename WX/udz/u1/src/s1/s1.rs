/*
Section work will be here , wills start with w1.rs and so on also they called from mod.rs
- Simiulates a decsk of playing cards
*/

#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};

// --- Main Function ---
pub fn s1_main() {
    greet();
}

// --- Sub Functions---

// Greet function

fn greet() {
    pswg("Section 1 Work - Deck of cards ".to_string())
}

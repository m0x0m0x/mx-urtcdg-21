/*
Section work will be here , wills start with w1.rs and so on also they called from mod.rs
- Simiulates a decsk of playing cards
- This will have the code for the deck
*/

#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};
use yansi::Paint;

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

#[derive(Debug)]
struct Deck {
    cards: Vec,
}

fn func1() {
    header("Deck Function");

    // Alternative method of declaring this Vector
    // let deck = Deck { cards: Vec::new() };

    // List of suits
    let suits = vec!["❤️", "♦️", "♣️", "♠️"];

    // List of Values

    // Doublet nested for loops

    let deck = Deck { cards: vec![] };

    println!("Deck = {:#?}", deck.green())
}

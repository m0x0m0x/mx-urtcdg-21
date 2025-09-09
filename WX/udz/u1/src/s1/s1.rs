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
    cards: Vec<String>,
}

fn func1() {
    header("Deck Function");

    // Alternative method of declaring this Vector
    // let deck = Deck { cards: Vec::new() };

    // List of suits
    let suits = ["❤️", "♦️", "♣️", "♠️"];

    // List of Values
    let values = ["A", "K", "Q", "J"];

    // Empty vector to store the cards
    let mut cards = vec![];

    // Doublet nested for loops
    for s in suits {
        for v in values {
            let card = format!("{}{}", v, s);
            cards.push(card);
        }
    }

    let deck = Deck { cards };

    println!("Deck = {:?}", deck.green())
}

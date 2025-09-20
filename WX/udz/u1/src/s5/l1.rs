/*
l1.ra
- This rust file is for writing learning patterns
*/

// -- Directives ---

#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};
use yansi::Paint;

// --- Main Function ---

pub fn s5_l1_main() {
    greet();
}

// --- Sub Functions---

fn greet() {
    let str = "Secton 5 - Learning l1.rs";
    pswg(str.to_string());
}

/*
Learning Enums
1. Fist we will visualize what the enum should look like
2. This is like defining 3 different structs
*/

#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, dirctor: String },
    Audibook { title: String },
}

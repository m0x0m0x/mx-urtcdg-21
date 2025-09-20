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
    let str = "
Secton 5 - Learning l1.rs \n
--- 
learning 
";

    pswg(str.to_string());
    pswg("What \n Mean \n this ".to_string())
}

/*
l1.rs -
1. All learning related code here
*/

// -- Directives ---
#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};
use yansi::Paint;

// --- Main Function ---

pub fn s7_l1_main() {
    greet();
}

// --- Sub Functions ---

fn greet() {
    pswg("s7/l1.rs - Learning related code here".to_string())
}

// --- Sub Functions ---

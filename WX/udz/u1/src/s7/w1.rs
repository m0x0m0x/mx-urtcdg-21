/*
Section 7 - Section 7 work here
- Focus is on making various logs
- Focus on handling errors
*/

#![allow(dead_code)]
#![allow(unused_imports)]

// --- Imports ---
use crate::utils::{header, pswg};
use rand::{rng, seq::SliceRandom};
use yansi::Paint;

// --- Main Function ---

pub fn s7_w1_main() {
    greet();
}

// --- Sub Functions ---

fn greet() {
    pswg("Sec7 - Section 7 work here".to_string())
}

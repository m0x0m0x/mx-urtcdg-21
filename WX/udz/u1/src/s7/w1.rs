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
use std::fs;
use yansi::Paint;

// --- Main Function ---

pub fn s7_w1_main() {
    greet();
    func1();
}

// --- Sub Functions ---

fn greet() {
    pswg("Sec7 - Section 7 work here".to_string())
}

// Open and reading the file

fn func1() {
    header("Opening and Reading the file");
    let file = fs::read_to_string("src/s7/s7logs.txt").unwrap();
    println!("{:#?}", file.yellow());
}

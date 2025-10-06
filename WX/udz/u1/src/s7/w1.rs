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
    // func1();
    // func2();
    func3();
}

// --- Sub Functions ---

fn greet() {
    pswg("Sec7 - Section 7 work here".to_string())
}

// Open and reading the file

fn func1() {
    header("Opening and Reading the file");
    let file = fs::read_to_string("src/s7/s7logs.txt").unwrap();

    // note this formatter is for debugging of data strcuts
    println!("{}", "---Printing With formatter---".on_blue());
    println!("{:#?}", file.yellow());

    // Normally print out to terminal done like this
    println!("{}", "---Printing w/o formatter ---".on_blue());
    println!("{}", file.blue());
}

// Same as above functions using match statement
fn func2() {
    header("Using match statement");

    match fs::read_to_string("src/s7/s7logs.txt") {
        Ok(file) => {
            println!("{}", "---Printing With formatter Characters---".on_blue());
            println!("{:#?}", file.len().yellow());
            println!("{}", "---Printing Full File---".on_blue());
            println!("{:#?}", file.yellow());
        }
        Err(e) => {
            println!("{}", "Error: ".red());
            println!("{}", e.to_string().red());
        }
    }
}

// test function for strings

// fn string_test(a: String, b: &String, c: &str) {}

fn func3() {
    header("Using match statement");

    // string_test(String::from("Panty"), &String::from("Panty"), "Panty");

    match fs::read_to_string("src/s7/s7logs.txt") {
        Ok(file) => {
            println!("{}", "---Printing With formatter Characters---".on_blue());
            println!("{:#?}", file.len().yellow());
            println!("{}", "---Printing Full File---".on_blue());
            println!("{:#?}", file.yellow());
        }
        Err(e) => {
            println!("{}", "Error: ".red());
            println!("{}", e.to_string().red());
        }
    }
}

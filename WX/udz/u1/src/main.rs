/*
Main entry point for the Rust application.
*/

mod utils;

use utils::{header, pswg};

fn main() {
    pswg("booty".to_string());
    header("booty");
}

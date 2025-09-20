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

    // Sub functions here
    pr_ab();
}

/*
Learning Enums
1. Fist we will visualize what the enum should look like
2. This is like defining 3 different structs
3. In a case where you need similar structs use enums
*/

#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String, year: i32 },
}

// Function to print out the enum
fn print_media(media: Media) {
    header("Printing Enums");
    println!("{:#?}", media.magenta())
}

// printing the audio book function
fn pr_ab() {
    println!("{}", "Printing the Audiobook".blue());

    // Audio Book
    let ab1 = Media::Audiobook {
        title: String::from("Booty Smelling"),
        year: 2099,
    };

    // Movie
    let mov1 = Media::Movie {
        title: String::from("Scat Mistress"),
        director: String::from("AssAddict"),
    };

    // Book
    let bok1 = Media::Book {
        title: String::from("BootyLicking"),
        author: String::from("Booty Addicts"),
    };

    print_media(ab1);
    print_media(mov1);
    print_media(bok1);
}

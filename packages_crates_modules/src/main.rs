/*** Learnings from the Book
A crate is the smallest amount of code that a Rust compiler considers, not a file.
If one file is passed to rustc, then it is considered a module made up of one file.

Crates can contain modules which can be present in different files.

Crates can come in two forms: a binary crate or a library crate
1. Library crate - no main function, not an executable.
2. Binary crate - main function with executable.

A package is a bundle of one or more crates that provide a set of functionality.
- Can contain as many binary crates as you like
- Only one library crate
- `cargo new` is used to create a crate

Package conventions:
- `main.rs` is usually the crate root / entry point of a binary crate with the same name as the package.
- if src contains `lib.rs`, the crate contains a library with the same name as the package

***/


//For more info about modules and how they are structured, reference modules_cheatsheet.md

//Lets create a few modules and packages to model a backyard
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

/*** The purpose of modules
Modules let us organize code within a crate for readability and easy reuse. Modules also allow us
to control the privacy of items, because code within a module is private by default.

Private items are internal implementation details not available for outside use. We can choose to make modules
and items within them public, which exposes them to allow external code to use and depend on them.
***/

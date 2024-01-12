# Modules Cheat Sheet
A quick reference on how modules, paths, the `use` keyword, and the `pub` keyword work in the compiler, as well as how most developers organize their code.

1. Start from the crate root
2. Declare modules
3. Declaring sub modules
4. Paths to code in modules
5. Private vs public
6. The use keyword

## Start from the crate root
When compiling a crate, compiler searches for the crate root, usually a `lib.rs` or a `main.rs` file for code to compile. (Searching for the entry point)

## Declaring Modules
In the crate root, which is usually `lib.rs` or `main.rs`, we can declare modules.

Suppose I declare a `garden` module with `mod garden`; The compiler will look for the module's code in these places:
1. Inline, within curly brackets that replace the semicolon following `mod garden`
2. In the file `src/garden.rs`
3. In the file `src/garden/mod.rs`

## Declaring Sub-modules:
In any file other than the crate root, you can declare submodules. Modules that parent modules depend upon. If we declare `mod vegetables` within module `garden`, we place it in `src/garden.rs`

The compiler will look for the submodule's code within the directory named for the parent module in these places: 
1. Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon.
2. In the file `src/garden/vegetables.rs`
3. In the file `src/garden/vegetabes/mod.rs`

## Paths to code in modules
Once a module is part of the crate, we can refer to that code in that module from anywhere else in the crate, as long as privacy rules allow.

For instance, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`

## Private vs Public
Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`.
To make items within a public module public as well, use the `pub` before their declarations.

In general rust assumes modules and methods within that module are private unless explicity declared as such.

## The use keyword
Within a scope, the `use` keyword creates shortcuts to items to reduce the repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a short cut by typing `use crate::garden::vegetables::Asparagus.`

Now, just refer to the type with just `Asparagus`
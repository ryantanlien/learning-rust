fn main() {
    let _s = String::from("hello"); // s is valid from this point forward

    // do stuff with s

    let s1 = String::from("hello");
    // let s2 = s1; 
    //line 7 is one of the issue of memory management, if Rust allows two pointers to the same variable,
    //it would result in a double free error.

    //Instead s2 now invalidates s1; we don't have a shared pointer.
    //In C++ we call this _moving_

    //If you want a copy, do it explicitly.
    let s2 = s1.clone();

    //Primitives types implement the Copy trait that allows assignment to clone
    //Any Type that implements the Drop trait cannot implement the Copy trait and vice versa
    
    println!("s1 = {}, s2 = {}", s1, s2);

    //after the function is done, scope is over and s is no longer valid
    //this is known as `drop`, similar to RAII in C++ where we deallocate resources
    //at the end of the object's lifetime. (Resource Acquisition Is Initialization
        
    ownership();
}


fn ownership() {
    let s = String::from("hello");

    //Function takes ownership of s through a move
    takes_ownership(s);

    //Produces a compile time error as s is dropped
    //println!("s = {}", s);

    let s1 = String::from("hi i am given back");
    println!("s1 = {}", takes_and_gives_back_ownership(s1));

    let x = 5;
    
    makes_copy(x);

    let mut s2 = String::from("hello");
    calculate_length(&s2); //s2 borrows the value s2 basically like takes_and_gives_back_ownership

    //change(&s); //This code produces a compile-time error as references are not mutable by default, use mutable references instead
    correct_change(&mut s2); //Have to declare s2 as mutable and pass a mutable reference to the change

    // let r1 = &mut s2;
    // // let r2 = &mut s2; //cannot perform a mutable borrow more than once. prevents data races

    // println!("{}, {}", r1, r2); //mutable borrows being used at the same time here -> compile time error 
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back_ownership(some_string: String) -> String {
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn return_tuple() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1); 
    //The &s1 syntax creates a reference that refers to s1 but does not own it.
    //Fairly similar to pointers in C++, but only the owner can drop it.

    //The action of creating a reference is known as _borrowing_

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn correct_change(some_string: &mut String) {
    some_string.push_str(", world");
}

//Rust prevents dangling references at compile time
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s //Dangles the reference as s is out of scope after the function
// }

fn slice() {
    let mut s = String::from("hello world");

    let word = first_word(&s); //word will get value 5;

    s.clear(); //using the slice version of first_word now introduces a compilation error

    println!("the first word is: {}", word);

    //word still has the value 5 here, but there's no more string that 
    //we could meaningfully use the value 5 with. word is now totally invalid.
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
   
    // for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return i;
    //     }
    // }
 
    //s.len();
    
    //version with string slicing    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; //slice operator
        }
    }

    &s[..]
}
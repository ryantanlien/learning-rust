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
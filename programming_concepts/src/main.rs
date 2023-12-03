fn main() {
    //The below should complain about x not being declared mut
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    //Expressing const in Rust
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    //Example of Shadowing in Rust and Scoping
    //Shadowing
    let x = 10;

    //Scoping
    {
        let x = 11;
        println!("The value of x is in the inner scope is: {x}") 
    }

    println!("The value of x is {x}");
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    //to get a specific value from a struct we must use dot notation.
    //we must also declare the user to be mutable.
    
    let mut user2 = User {
        active: true, 
        username: String::from("someusername1231"),
        email: String::from("someone1@example.com"),
        sign_in_count: 2,
    };

    user2.sign_in_count = 3;
    
    //using struct update syntax, when we want to change only some fields of a struct, but rest is same
    let user2 = User {
        email: String::from("someusername1232"),
        ..user1
    };

    //tuple struct demonstration
    let black = Color(0,0,0);
    let origin = Color(0,0,0);

    //unit-like structs, useful when you want a trait on a struct without any data
    let subject = AlwaysEqual;

    //for now all structs defined above own their own data, but its possible for structs to contain references, though requiring lifetimes.
    //More will be elaborated on in Chapter 10.
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//unit-like struct
struct AlwaysEqual;
 
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

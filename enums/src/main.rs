fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    //enums with same inner type as String
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    //enums with different inner type
    let home1 = IpAddrV1::V4(127, 0, 0, 1);
    let loopback = IpAddrV1::V6(String::from("::1"));

    //enum message with function
    let m = Message::Write(String::from("hello"));
    m.call();
    match_example();
}

fn route(ip_kind: IpAddrKind) {}

//Basic enum type
enum IpAddrKind {
    V4,
    V6,
}

//We can place data inside enums as well, to better use enums as a Data Structure,
//this way we do not need extra structs to express different types

enum IpAddr {
    V4(String),
    V6(String),
}

//each enum can have different types
//version four ip addresses will always have four numeric components btw 0 and 255
//if we wanted to store V4 addresses as four u8 values but V6 addresses as one String value, we can do the below:
enum IpAddrV1 {
    V4(u8, u8, u8, u8),
    V6(String),
}

//you can even put structs

//Rust's version of polymorphism must use enums
//different kinds of messages written with enums:
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//the same kinds written with structs instead:
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); //tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct

//The limitation of the struct version is that we won't have a compilable version
//of the structs that could share the same methods, but we can with enums:
//
impl Message {
    fn call(&self) {
        // method body would be defined here
        //self to get the value that we called the method on.
    }
}

//Case Study Of Option<T>
//serves as a replacement for null reference, but compilable
//that Option can be Nothing or Something, and both cases must be handled,
//this way, we will never have a case where a null reference craaashes the program,
//as all nothing values must be explicitly handled

fn match_example() {
    #[derive(Debug)]
    enum UsState { 
        Alabama,
        Alaska
    }

    //Using nested structure in enums for examples of patterns that bind to values
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState)
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            //add a variable called state to the pattern in the match statement that matches the values 
            // of the variant Coin::Quarter
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alabama));
}
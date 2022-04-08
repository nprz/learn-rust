fn main() {
    /* 7pm - complete chapter 6 by 9pm */

    // Chapter 6 - Enums and Patter Matching

    // Enums allow you to define a type by enumerating its possible variants
    // Chapter objectives:
    // define and show how an enum can encode meaning along with data ✅
    // explore the enum Option, which expresses a value can be either something or nothing ✅
    // look at pattern matching with the match expression and how it makes it easy to 
    // run different code for different values of an enum
    // cover how the if let construct is another convenient and concise idiom avialable to handle enums

    // 6.1 Defining an Enum
    // Any IP address can be either a version four or a version six address, but not both at the same time.
    // we can put data directly into each enum variant
    enum IpAddrKind {
        V4(String),
        V6(String),
    }

    // the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    fn route(ip_kind: IpAddrKind) {}

    // each variant can have different types and amounts of associated data
    enum IpAddrKind2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // we’re also able to define methods on enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }   

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    // it is possible to define the above as: 
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // using a enum allows us to more easily define a function that can take any kind of these Messages

    // Rust does not have null vars, instead the Option enum is available.

    // this will not run (now it will since calling unwrap()) since you have to convert an Option<T> to a T before you can perform T operations with it.
    // Eliminating the risk of incorrectly assuming a not-null value helps you to be more confident in your code
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z = y.unwrap() + x;

    println!("z: {}", z);

    // 6.2 - The Match Control Flow
    // match allows you to compare a value against a series of patterns and then execute code based on which pattern matches
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }


    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));

    // values 
}   

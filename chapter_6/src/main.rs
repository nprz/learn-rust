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

    // the inner state value of the enum can be accessed like so,
    // docs refer to this as binding the values that mathc the pattern
    value_in_cents(Coin::Quarter(UsState::Alaska)); 

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            // The i binds to the value contained in Some, so i takes the value 5
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("The value of six: {:?}. The value of none {:?}", six, none);

    // Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.
    // other can be used as a catch all value that covers all other possibilities. Note the catch all arm must be 
    // put last because the patterns are evaluated in order.
    // _ is a special pattern that matches any value and does not bind to that value. This tells rust we are not going to use the value

    // 6.3 Concise Control Flow with id let
    // The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern
    // while ignoring the rest
    
    // this
    let config_max = Some(8u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // can be shortened to
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }
    
    // The code in the if let block isn’t run if the value doesn’t match the pattern.
    // you can think of `if let` as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values
    // We can include an else with an if let. else works the same as _ when using match

}   

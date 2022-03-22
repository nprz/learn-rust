fn main() {
    // 3.1

    // the mut keyword must be used to alter a variable's value
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants are always immutable  
    // consts must be annotated with a type
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);


    // declaring a new varible with the same name as a previous varible is known as shadowing
    // shadowing allows a variable to change its type, which is not allowed with mut
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);
     
    // 3.2

    // Rust is a statically typed language, which means that it must know the types of all variables at compile time

    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters

    // specify char literals with single quotes, as opposed to string literals, which use double quotes.

    // Tuples have a fixed length: once declared, they cannot grow or shrink in size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_a, b, _c) = tup;
    println!("The value of b is: {}", b);
    println!("The first value in the tuple is: {}", tup.0);
    
    /* 
        The tuple without any values, (), is a special type that has only one value, also written (). 
        The type is called the unit type and the value is called the unit value. 
        Expressions implicitly return the unit value if they don’t return any other value.
    */

    // arrays in Rust have a fixed length.
    // A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size
    let ary = [1, 2, 3, 4, 5];
    let first = ary[0];
    println!("The first value in the array is: {}", first);

    // 3.3

    // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value
    fn plus_one(x: i32) -> i32 {
        x + 1  
    }

    let num = plus_one(5);

    println!("The value of num is {}", num);

    // 3.5 

    // Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition.
    // blocks of code evaluate to the last expression in them

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // loop
    // You can place the break keyword within the loop to tell the program when to stop executing the loop.
    // continue tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration
    // You can optionally specify a loop label on a loop that we can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop
    // you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    // for loops examples
    let vals = [10, 20, 30, 40, 50];

    for element in vals {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}


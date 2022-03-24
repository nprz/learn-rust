fn main() {

    // 4.1 - What is Ownership?
    // In rust, memory is managed through a system of ownership with a set of rules that the compiler checks.
    // The stack stores values in the order it gets them and removes the values in the opposite order LIFO
    // when you put data on the heap, you request a certain amount of space
    // The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer

    // Rules of ownership: 
    // Each value in Rust has a variable that’s called its owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // String literals are immutable


    // The double colon :: operator allows us to namespace this particular from function under the String type 
    // rather than using some sort of name like string_from

    // In order to support a mutable string we need:  
    // 1) to allocate memory at runtime
    // 2) return allocated memory when we are finished with the string

    // String::from allocated the needed memory
    // When a variable goes out of scope, Rust calls a special function for us. This function is called drop
    // Rust calls drop automatically at the closing curly bracket.

    // scalar values perform a copy. the value 5 will be copied into variable y.
    // both values are held on the stack
    let x = 5;
    let y = x;
    println!("x: {}, y:{}", x, y);

    // Objects perform a move. s1 no longer stores a pointer once assigned to s2.
    // in addition to a pointer, the length (# of bytes in use), and capacity (# of bytes available for use) are also
    // stored in the s1 variable.
    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack like integers are)
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2: {}", s2);

    // if you want to clone a value stored on the heap you should call clone: 
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    // Ownership and functions
    let s5 = String::from("hello");

    takes_ownership(s5); // takes_ownership is now the owner of s5, s5 is dropped

    let z = 5;

    makes_copy(z); // z is not dropped, a copy is made inside makes_copy

    println!("z: {}", z);

    // ownership initially begins in the gives_ownership fn, but is passed to the s6 after the fn completes.
    let s6 = gives_ownership();
    println!("s6: {}", s6);

    // 4.2 - References and Borrowing

    // references allow you to refer to some value without taking ownership of it
    let mut s7 = String::from("hello");
    let len = calculate_length(&s7);
    println!("The length of '{}' is {}.", s7, len);

    // references are immutable by default, to make references mutable you must specify by adding the mut keyword to the var, paramters, and arguments 
    change(&mut s7);
    println!("{}.", s7);

    // you can only borrow one mutable reference in a scope. decalring another reference mutable or immutable is not allowed
    // The benefit of having this restriction is that Rust can prevent data races at compile time
    let r1 = &s7; // no problem
    let r2 = &s7; // no problem
    println!("{} and {}", r1, r2); // <-- the scope for r1 and r2 ends here.
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s7; // no problem
    println!("{}", r3);

    // dangling pointer: a pointer that references a location in memory that may have been given to someone else -- by freeing
    // some memory while preserving a pointer to that memory.
    // In Rust, the compiler guarantees that references will never be dangling references
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(mut some_integer: i32) {
    some_integer = some_integer + 1; 
    println!("some_integer: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn main() {
    // Chapter 5 - Structs: 
    // A struct, or structure, is a custom data type that
    // lets you package together and name multiple related values that make up a meaningful group.
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1
        }
    }

    // if we want to change an attribute of a struct we must declare it as mutable
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someusername"));

    // We can no longer use user1 since the username attribute has been moved to user2
    // user2 now owns the pointer to username, 
    let user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1
    };

    user1.email = String::from("newemail@example.com");

    println!("User 1 email: {}, User 2 email: {}", user1.email, user2.email);

    // this is a tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("The first value in the tuple is: {}", black.0);

    // It’s possible for structs to store references to data owned by something else, 
    // but to do so requires the use of lifetimes. 
    // Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
    // Lifetimes will allow us to use &str type rather than &String
    // This will be taught in chapter 10.

    // 5.2 - An Example Program Using Structs
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle {
        // &self is shorthand for self: &Self
        // If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter
        // Having a method that takes ownership (not borrowing) of the instance by using just self as the first parameter is rare
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn width(&self) -> bool {
            self.width > 0
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size
            }
        }
    }

    let rect1 = Rectangle {
        height: 30,
        width: 50
    };

    let rect2 = Rectangle {
        height: 10,
        width: 40
    };

    println!("the area of rectangle is {} square pixels", rect1.area());
    println!("the rectangle has a nonzero width: {}", rect1.width());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    // the dbg! macro can be used to print standard errors to the stderr
    // the dbg! macro takes ownership of an expression, prints the file and the line number along with the output of the expression
    // and then returns ownership of the value.

    // 5.3 - Method syntax
    // Methods are defined within the context of a struct, their first parameter is always self
    // when we give methods with the same name as a field we want it to only return the value in the field and do nothing else. Methods like this are called getters
    // all methods are associated functions. not all associated functions are methods. associated functions that do not take &self as the first parameter are not methods.
    // associated functions that are not methods are often constructors

    // To call an associated function, we use the :: syntax with the struct name
    // This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces created by modules
    let sq = Rectangle::square(5);
    println!("the area of rectangle is {} square pixels", sq.area());

    // Each struct is allowed to have multiple impl blocks
    // it makes more sense to use multiple impl blocks when defining generic types and traits
}


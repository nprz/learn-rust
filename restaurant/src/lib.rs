mod front_of_house {
    // Modules can also hold definitions for other items, such as structs, enums, constants, traits, or—as in this example functions.
    
    mod hosting {
        fn add_to_waitlist() {}
    }

}

pub mod fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

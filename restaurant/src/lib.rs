mod front_of_house {
    // Modules can also hold definitions for other items, such as structs, enums, constants, traits, or—as in this example functions.
    
    // need to add pub so eat_at_restaurant can access hosting
    pub mod hosting {
    // again need to add pub so eat_at_restaurant can access add_to_waitlist
       pub fn add_to_waitlist() {}
    }

}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // if we make an enum public, all of its variants are then public, whereas in structs
    // fields must be declared pub

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

// The front_of_house module isn’t public, but because the eat_at_restaurant function is defined in the same module as front_of_house we can refer to front_of_house from eat_at_restaurant
pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change mind about what kind of toast we want
    meal.toast = String::from("Wheat");
    println!("I's like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // because back_of_house::Breakfast has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast
    // we would not be able to create an instance of Breakfast here since it has a private field

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

        // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

}

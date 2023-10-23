fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();

        // Relative path with super keyword (parent module)
        super::deliver_order();
    }

    fn cook_order() {}
}

mod front_of_house;

// Re-exporting names with pub use
pub use crate::front_of_house::hosting::add_to_waitlist;

// now external code can call the add_to_waitlist function using hosting::add_to_waitlist
// and not like this: restaurant::front_of_house::hosting::add_to_waitlist

// in the customer module super::hosting references this:
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path with crate keyword (root)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path with module name
    front_of_house::hosting::add_to_waitlist();

    // Relative path with use keyword
    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();

    mod customer {
        pub fn eat_at_restaurant() {
            // use only brings the function into the current scope
            // hosting::add_to_waitlist(); // Error: use of undeclared crate or module `hosting`

            // 1. fix: move use to parent scope:
            use crate::front_of_house::hosting;
            hosting::add_to_waitlist();

            // 2. fix: use super keyword:
            super::hosting::add_to_waitlist();
        }
    }

    // Not idiomatic
    use crate::front_of_house::hosting::add_to_waitlist;
    add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); // Error: private field

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

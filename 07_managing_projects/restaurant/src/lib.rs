// our example starts here
mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super is same as ../ in path terminology!
        super::deliver_order();
    }
    fn cook_order() {}

    // Breakfast becomes public but its fields remain private
    pub struct Breakfast {
        pub toast: String,      // we let customer choose the type of bread!
        seasonal_fruit: String  // This field stays hidden
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    // For enumeration if we set pub, each item of the enum is public!
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// bringing hosting into the scope --> we can use shorter form hosting::add_to_waitlist()
//  works only in current scope!
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    hosting::add_to_waitlist();
}

use std::fmt;
use std::io;

// here we import parent module to distinguish between Result functions
// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

// // we can use alias with word 'as'
// use std::io as IoResult
// fn function3() -> IoResult<()> {}
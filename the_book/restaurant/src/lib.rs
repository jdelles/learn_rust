mod front_of_house;
pub use crate::front_of_house::hosting;

use rand::{CryptoRng, ErrorKind::Transient, Rng};
use std::fmt::Result;
use std::io::Result as IoResult;
use std::io::*;
use std::io::{self, Write};

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1..101);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Wheat");
    meal.toast = String::from("Rye");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    fn function1() -> Result {
        Ok(())
    }
    fn function2() -> IoResult<String> {
        Ok(String::from("Hello"))
    }
}

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
        super::front_of_house::hosting::add_to_waitlist();
    }

    fn cook_order() {
        println!("Cook order");
    }
}

fn serve_order() {}

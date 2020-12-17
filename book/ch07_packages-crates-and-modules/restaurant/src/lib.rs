#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use self::back_of_house::Appetizer;
use crate::front_of_house::hosting;

#[allow(unused_variables)]
pub fn eat_at_restaurant() {
    // `use` directive enables us to use this
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // field `...` of struct `...` is private
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let order3 = Appetizer::Soup;
}

#[allow(dead_code)]
mod front_of_house {

    pub mod hosting {

        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {

        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

#[allow(dead_code)]
mod back_of_house {

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}

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
}

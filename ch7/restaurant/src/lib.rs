mod front_of_house;

fn serve_order() {
    crate::front_of_house::serving::serve_order();
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub appetizer: Appetizer,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                appetizer: Appetizer::Soup,
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
        super::serve_order();
    }

    fn cook_order() {
        println!("Cooking");
    }
}

// Absolute path
use crate::front_of_house::hosting;
// Relative path
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Using `use`
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);
    meal.appetizer = back_of_house::Appetizer::Soup;
    meal.appetizer = back_of_house::Appetizer::Salad;

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

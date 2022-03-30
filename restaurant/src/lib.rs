// use std::io;
// use std::io::Write;
use std::collections::*;
use std::io::{self, Write};

mod front_of_house;

mod back_of_house {
	pub enum Appetizer {
		Soup,
		Salad,
	}

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

	fn fix_incorrect_order() {
		cook_order();
		super::serve_order();
	}

	fn cook_order() {}
}

fn serve_order() {}

pub fn eat_at_breakfast() {
	let mut meal = back_of_house::Breakfast::summer("Rye");

	meal.toast = String::from("Wheat");
	println!("I'd like {} toast please", meal.toast);
}

// use crate::front_of_house::hosting;
pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
	let order1 = back_of_house::Appetizer::Soup;
	let order2 = back_of_house::Appetizer::Salad;

	// Absolute path
	crate::front_of_house::hosting::add_to_waitlist();

	// Relative path
	front_of_house::hosting::add_to_waitlist();

	hosting::add_to_waitlist();
	// hosting::seat_at_table();
}

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}

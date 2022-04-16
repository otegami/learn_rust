// use std::net::IpAddr;

// Creating Custom Types for Validation
pub struct Guess {
	value: i32,
}

impl Guess {
	pub fn new(value: i32) -> Guess {
		if value < 1 || value > 100 {
			panic!("Guess value must be between 1 and 100, got {}.", value);
		}

		Guess { value }
	}

	pub fn value(&self) -> i32 {
		self.value
	}
}

fn main() {
	loop {
		let guess = String::from("1026");

		let guess: i32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue
		};

		if guess < 1 || guess > 100 {
			println!("The secret number will be between 1 and 100.");
			continue;
		}
	}
}

// Guidelines for Error Handling

// // Cases in Which You Have More Information Than the Compiler
// fn main() {
// 	let home: IpAddr = "127.0.0.1".parse().unwrap();
// }

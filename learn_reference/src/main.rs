// Generic Type Parameters, Trait Bounds, and Lifetimes Together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
	T: Display,
{
	println!("Announcement! {}", ann);
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn main() {}

// The Static Lifetime
// fn main() {
// 	let s: &'static str = "I have a static lifetime.";
// }

// // Lifetime Annotations in Method Definitions
// struct ImportantExcerpt<'a> {
// 	part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
// 	fn level(&self) -> i32 {
// 		3
// 	}

// 	fn announce_and_return_part(&self, announcement: &str) -> &str {
// 		println!("Attention please: {}", announcement);
// 		self.part
// 	}
// }

// fn main() {}

// fn main() {
// 	let novel = String::from("Call mew Ishamel. Some years ago...");
// 	let first_sentence = first_word(&novel);
// 	// let first_sentence = novel.split('.').next().expect("Could not find a '.'");
// 	let i = ImportantExcerpt {
// 		part: first_sentence,
// 	};
// 	println!("{}", i.part);
// }

// fn first_word(s: &str) -> &str {
// 	let bytes = s.as_bytes();

// 	for (i, &item) in bytes.iter().enumerate() {
// 		if item == b'w' {
// 			return &s[0..i];
// 		}
// 	}

// 	&s[..]
// }

// // Generic Lifetimes in Functions
// fn main() {
// 	let string1 = String::from("long string is long");
// 	let result;
// 	{
// 		let string2 = String::from("xyz");
// 		result = longest(string1.as_str(), string2.as_str());
// 	}

// 	println!("The longest string is {}", result);
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
// 	if x.len() > y.len() {
// 		x
// 	} else {
// 		y
// 	}
// }

// fn main() {
// 	let r;
// 	{
// 		let x = 5;
// 		r = x;
// 	}
// 	println!("r: {}", r);
// }

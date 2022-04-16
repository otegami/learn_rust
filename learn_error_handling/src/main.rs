use std::error::Error;
use std::fs::File;
// use std::fs;
// use std::io;
// use std::fs::File;
// use std::io::Read;
// use std::io::ErrorKind;

// Where The ? Operator Can Be Used
fn main() -> Result<(), Box<dyn Error>> {
	File::open("hello.txt")?;
	Ok(())
}

fn last_char_of_first_line(text: &str) -> Option<char> {
	text.lines().next()?.chars().last()
}

// Propagating Errors
// fn read_username_from_file() -> Result<String, io::Error> {
// 	fs::read_to_string("hello.txt")
// }

// fn read_username_from_file() -> Result<String, io::Error> {
// 	let mut s = String::new();
// 	File::open("hello.txt")?.read_to_string(&mut s)?;
// 	Ok(s)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
// 	let mut f = File::open("hello.txt")?;
// 	let mut s = String::new();
// 	f.read_to_string(&mut s)?;
// 	Ok(s)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
// 	let f = File::open("hello.txt");

// 	let mut f = match f {
// 		Ok(file) => file,
// 		Err(e) => return Err(e)
// 	};

// 	let mut s = String::new();

// 	match f.read_to_string(&mut s) {
// 		Ok(_) => Ok(s),
// 		Err(e) => Err(e)
// 	}
// }

// // Shortcuts for Panic on Error: unwrap and expect
// fn main() {
// 	// let f = File::open("hello.txt").unwrap();
// 	let f = File::open("hello.txt").expect("Failed to open hello.txt");
// }

// // Recoverable Errors with Result
// fn main() {
// 	let f = File::open("hello.txt");

// 	let f = match f {
// 		Ok(file) => file,
// 		Err(error) => match error.kind() {
// 			ErrorKind::NotFound => match File::create("hello.txt") {
// 				Ok(fc) => fc,
// 				Err(e) => panic!("Problem creating the file: {:?}", e)
// 			},
// 			other_error => {
// 				panic!("Problem opening the file: {:?}", other_error)
// 			}
// 		}
// 	};
// }

// // Using a panic! Backtrace
// fn main() {
// 	let v = vec![1, 2, 3];

// 	v[99];
// }

// // Unrecoverable Errors with panic!
// fn main() {
//   panic!("crash and burn");
// }

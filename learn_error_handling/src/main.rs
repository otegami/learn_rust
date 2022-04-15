use std::fs::File;
// use std::io::ErrorKind;

// Shortcuts for Panic on Error: unwrap and expect
fn main() {
	// let f = File::open("hello.txt").unwrap();
	let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

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

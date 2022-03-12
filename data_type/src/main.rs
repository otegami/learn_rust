// in from -2 ** (n - 1) to 2 ** (n - 1)
//     i8: from -128 to 128
// un from 0 to 2 ** n
//     u8: from 0 to 256

// let sum = 5 + 10;
// let dfference = 95.4 - 4.3;
// let product = 4 * 30;
// let quotient = 56.7 / 32.3;
// let floored = 2 / 3;
// let remainder = 43 % 5;

// let tup: (i32, f64, u8) = (500, 6.4, 1);

// let tup: (i32, f64, u8) = (500, 6.4, 1);
// let five_hundred = tup.0;
// let six_pint_four = tup.1;
// let one = tup.2;

// println!("The value of one is: {}", one);

// let a: [i32; 5] = [1, 2, 3, 4, 5];

// let a = [1, 2, 3, 4, 5];

// let first = a[0];
// let second = a[1];

use std::io;

fn main() {
	let a = [1, 2, 3, 4, 5];

	println!("Please enter an array index.");

	let mut index = String::new();

	io::stdin()
		.read_line(&mut index)
		.expect("Failed to read line");

	let index: usize = index
		.trim()
		.parse()
		.expect("Index entered was not a number");

	let element = a[index];

	println!(
		"The value of the element at index {} is: {}",
		index, element
	)
}

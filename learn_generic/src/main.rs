// Performance of Code Using Generics
// enum Option_i32 {
// 	Some(i32),
// 	None,
// }

// enum Option_f64 {
// 	Some(f64),
// 	None,
// }

// fn main() {
// 	let integer = Option_i32::Some(5);
// 	let float = Option_f64::Some(5.0);
// }

// In Method Definitions
// struct Point<X1, Y1> {
// 	x: X1,
// 	y: Y1,
// }

// impl<X1, Y1> Point<X1, Y1> {
// 	fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
// 		Point {
// 			x: self.x,
// 			y: other.y,
// 		}
// 	}
// }

// fn main() {
// 	let p1 = Point { x: 5, y: 10.4 };
// 	let p2 = Point { x: "Hello", y: 'c' };

// 	let p3 = p1.mixup(p2);

// 	println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// struct Point<T> {
// 	x: T,
// 	y: T,
// }

// impl<T> Point<T> {
// 	fn x(&self) -> &T {
// 		&self.x
// 	}
// }

// impl Point<f32> {
// 	fn distance_from_origin(&self) -> f32 {
// 		(self.x.powi(2) + self.y.powi(2)).sqrt()
// 	}
// }

// fn main() {
// 	let p = Point { x: 5, y: 10 };
// 	println!("p.x = {}", p.x());
// }

// In Struct Definitions
// struct Point<T, U> {
// 	x: T,
// 	y: U,
// }

// fn main() {
// 	let integer = Point { x: 5, y: 10 };
// 	let float = Point { x: 1.0, y: 4.0 };
// 	let wont_work = Point { x: 5, y: 4.0 };
// }

// In Function Definitions
// fn largest_i32(list: &[i32]) -> i32 {
// 	let mut largest = list[0];

// 	for &item in list {
// 		if item > largest {
// 			largest = item;
// 		}
// 	}

// 	largest
// }

// fn largest_char(list: &[char]) -> char {
// 	let mut largest = list[0];

// 	for &item in list {
// 		if item > largest {
// 			largest = item;
// 		}
// 	}

// 	largest
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
	let mut largest = list[0];

	for &item in list {
		if item > largest {
			largest = item
		}
	}

	largest
}

fn main() {
	let numnber_list = vec![34, 50, 25, 100, 65];

	let result = largest(&numnber_list);
	println!("The largest number is {}", result);

	let char_list = vec!['y', 'm', 'a', 'q'];
	let result = largest(&char_list);

	println!("The largest char is {}", result)
}

// // Removing Duplication by Extracting a Function
// fn largest(list: &[i32]) -> i32 {
// 	let mut largest = list[0];

// 	for &item in list {
// 		if item > largest {
// 			largest = item;
// 		}
// 	}

// 	largest
// }

// fn main() {
// 	let number_list = vec![34, 50, 25, 100, 65];
// 	let result = largest(&number_list);

// 	println!("The largest number is {}", result);

// 	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
// 	let result = largest(&number_list);

// 	println!("The largest number is {}", result);
// }

// fn main() {
// 	let number_list = vec![34, 50, 25, 100, 65];
// 	let mut largest = number_list[0];

// 	for number in number_list {
// 		if number > largest {
// 			largest = number;
// 		}
// 	}

// 	println!("The largest number is {}", largest);
// }

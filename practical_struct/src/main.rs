#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size,
		}
	}

	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn width(&self) -> bool {
		self.width > 0
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};
	let rect2 = Rectangle {
		width: 10,
		height: 40,
	};
	let rect3 = Rectangle {
		width: 60,
		height: 45,
	};
	let rect4 = Rectangle::square(70);

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
	println!("Can rect4 hold rect3? {}", rect4.can_hold(&rect3));
}

// fn main() {
// 	let rect1 = Rectangle {
// 		width: 30,
// 		height: 50,
// 	};

// 	if rect1.width() {
// 		println!("The rectangle has a nonzero width; it is {}", rect1.width);
// 	}

// 	println!(
// 		"The area of the rectangle is {} square pixedls",
// 		rect1.area()
// 	);

// 	// なぜ、 area 関数の引数で & pointer を pointer 参照するようにしないと、
// 	// 下記のように参照する際に、 ownership を取られてしまい参照できなくなる
// 	// println!("{:#?}", rect1);
// }

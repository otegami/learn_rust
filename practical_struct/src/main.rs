#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	// let rect1 = Rectangle {
	// 	width: 30,
	// 	height: 50,
	// };

	// println!(
	// 	"The area of the rectangle is {} square pixedls",
	// 	area(&rect1)
	// );

	// なぜ、 area 関数の引数で & pointer を pointer 参照するようにしないと、
	// 下記のように参照する際に、 ownership を取られてしまい参照できなくなる
	// println!("{:#?}", rect1);

	let scale = 2;
	let rect1 = Rectangle {
		width: dbg!(30 * scale),
		height: 50,
	};

	dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}

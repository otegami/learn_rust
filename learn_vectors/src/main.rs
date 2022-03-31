// fn main() {
// 	let v: Vec<i32> = Vec::new();
// 	let v = vec![1, 2, 3];
// }

// fn main() {
// 	let mut v = Vec::new();
// 	v.push(5);
// }

// fn main() {
// 	let v = vec![1, 2, 3, 4, 5];

// 	let third: &i32 = &v[2];
// 	println!("The third element is {}", third);

// 	match v.get(2) {
// 		Some(third) => println!("The third element is {}", third),
// 		None => println!("There is no third element"),
// 	}
// }

// fn main() {
// 	let v = vec![1, 2, 3, 4, 5];

// 	let done_not_exist = &v[100];
// 	println!("The one hundred element is {}", done_not_exist);

// 	match v.get(100) {
// 		Some(one_hundred) => println!("The one hundredth element is {}", one_hundred),
// 		None => println!("There is no one hundredth element"),
// 	}
// }

// fn main() {
// 	// collections が確保するメモリが可変長だから、
// 	// 例え参照先が異なっても、後に再度メモリ配置を行う可能性があるから
// 	let mut v = vec![1, 2, 3, 4, 5];
// 	let first = &v[0];

// 	v.push(6);

// 	println!("The first elementis: {}", first);
// }

// fn main() {
// 	let mut v = vec![100, 32, 57];

// 	for i in &mut v {
// 		*i += 50;
// 		println!("{}", i);
// 	}
// }

fn main() {
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Float(10.12),
		SpreadsheetCell::Text(String::from("blue")),
	];
}

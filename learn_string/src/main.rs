// fn main() {
// 	let mut s = String::new();
// 	let data = "initial contents";
// 	let s = String::from(data);
// 	let s = data.to_string();

// 	let s = "initial contents".to_string();
// }

// fn main() {
// 	// let mut s = String::from("foo");
// 	// s.push_str("bar");

// 	// let mut s1 = String::from("foo");
// 	// let s2 = "bar";
// 	// s1.push_str(s2);
// 	// println!("s2 is {}", s2);

// 	let mut s = String::from("lo");
// 	s.push('l');
// }

// fn main() {
// 	let s1 = String::from("Hello, ");
// 	let s2 = String::from("world!");
// 	let s3 = s1 + &s2;

// 	println!("s1 is {}", s1);
// }

// fn main() {
// 	let s1 = String::from("tic");
// 	let s2 = String::from("tac");
// 	let s3 = String::from("toe");

// 	// let s = s1 + "-" + &s2 + "-" + &s3;
// 	let s = format!("{}-{}-{}", s1, s2, s3);

// 	println!("s is {}", s);
// 	println!("s1 is {}", s1);
// 	println!("s2 is {}", s2);
// 	println!("s3 is {}", s3);
// }

fn main() {
	// let s1 = String::from("Hola");
	// let h = &"Hola"[0];

	// let hello = "Здравствуйте";
	// let answer = &hello[0];

	for c in "おはよう".chars() {
		println!("{}", c);
	}
}

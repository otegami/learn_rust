// enum IpAddrKind {
// 	V4(u8, u8, u8, u8),
// 	V6(String),
// }

// struct IpAddr {
// 	kind: IpAddrKind,
// 	address: String,
// }

// fn main() {
// let home = IpAddr {
// 	kind: IpAddrKind::V4,
// 	address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
// 	kind: IpAddrKind::V6,
// 	address: String::from("::1"),
// };

// route(IpAddrKind::V4);
// route(IpAddrKind::V6);

// 	let home = IpAddrKind::V4(127, 0, 0, 1);
// 	let loopback = IpAddrKind::V6(String::from("::1"));
// }

// fn route(ip_kind: IpAddrKind) {}

// enum Message {
// 	Quit,
// 	Move { x: i32, y: i32 },
// 	Write(String),
// 	ChangeColor(i32, i32, i32),
// }

// impl Message {
// 	fn call(&self) {}
// }

// fn main() {
// 	let m = Message::Write(String::from("hello"));
// 	m.call();
// }

enum Option<T> {
	None,
	Some(T),
}

fn main() {
	// let some_number = Some(6);
	// let some_string = Some("a string");

	// let absent_number: Option<i32> = None;

	let x: i8 = 5;
	let y: Option<i8> = Some(5);

	let sum = x + y;
}

struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
	// let mut user1 = User {
	// 	email: String::from("someone_email@example.com"),
	// 	username: String::from("some_user_name123"),
	// 	active: true,
	// 	sign_in_count: 1,
	// };
	let mut user1 = build_user(
		String::from("someone_email@example.com"),
		String::from("some_user_name123"),
	);

	user1.email = String::from("another_email@example.com");

	// let user2 = User {
	// 	active: user1.active,
	// 	username: user1.username,
	// 	email: String::from("another@example.com"),
	// 	sign_in_count: user1.sign_in_count
	// };

	let user2 = User {
		email: String::from("another@example.com"),
		..user1
	};

	println!("{}", user1.email);

	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);

	let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
	User {
		email,
		username,
		active: true,
		sign_in_count: 1,
	}
}

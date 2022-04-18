pub trait Summary {
	fn summarize_author(&self) -> String;
	fn summarize(&self) -> String {
		String::from("(Read more...)")
	}
}
pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}
impl Summary for NewsArticle {
	fn summarize_author(&self) -> String {
		format!("{}", self.author)
	}
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}
pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}
impl Summary for Tweet {
	fn summarize_author(&self) -> String {
		format!("@{}", self.username)
	}
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

// pub fn notify(item: &impl Summary) {
// 	println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item: &T) {
// 	println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary + Display>(item: &T) {
// 	println!("Breaking news! {}", item.summarize());
// }

// Clearer Trait Bounds with where Clauses

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// fn some_function<T, U>(t: &T, u: &U) -> i32
// 	where T: Dsiplay + Clone,
// 				U: Clone + Debug
// 	{
// 	}

// fn returns_summarizable() -> impl Summary {
// 	Tweet {
// 		username: String::from("horse_ebooks"),
// 		content: String::from("of course, as you probably already know, people"),
// 		reply: false,
// 		retweet: false,
// 	}
// }

// fn returns_summarizable(switch: bool) -> impl Summary {
// 	if switch {
// 		NewsArticle {
// 			headline: String::from("Penguins win the Stanley Cup Championship!"),
// 			location: String::from("Pittsburgh, PA, USA"),
// 			author: String::from("Iceburgh"),
// 			content: String::from(
// 				"The Pittsburgh Penguins once again are the best \
// 							 hockey team in the NHL.",
// 			),
// 		}
// 	} else {
// 		Tweet {
// 			username: String::from("horse_ebooks"),
// 			content: String::from("of course, as you probably already know, people"),
// 			reply: false,
// 			retweet: false,
// 		}
// 	}
// }

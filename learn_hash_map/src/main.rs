use std::collections::HashMap;

// Updating a Value Based on the Old Value
fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map)
}

// // Only Inserting a Value If the Key Has No Value
// fn main() {
// 	let mut scores = HashMap::new();

// 	scores.insert(String::from("Blue"), 10);

// 	scores.entry(String::from("Yello")).or_insert(40);
// 	scores.entry(String::from("Blue")).or_insert(50);

// 	println!("{:?}", scores)
// }

// // Overwriting a Value
// fn main() {
// 	let mut scores = HashMap::new();

// 	scores.insert(String::from("Blue"), 10);
// 	scores.insert(String::from("Blue"), 25);

// 	println!("{:?}", scores)
// }

// fn main() {
// 	let field_name = String::from("Favorite color");
// 	let field_value = String::from("Blue");

// 	let mut map = HashMap::new();
// 	map.insert(field_name, field_value);

// 	println!("{}, {}", field_name, field_value);
// }

// fn main() {
// 	let mut scores = HashMap::new();

// 	scores.insert(String::from("Blue"), 10);
// 	scores.insert(String::from("Yellow"), 50);

// 	// let team_name = String::from("Blue");
// 	// let score = scores.get(&team_name);

// 	// println!("{}, {:?}", team_name, score);

// 	for (key, value) in &scores {
// 		println!("{}: {}", key, value);
// 	}

// 	// let teams = vec![String::from("Blue"), String::from("Yello")];
// 	// let initial_scores = vec![10, 20];

// 	// // {"Yello": 20, "Blue": 10}
// 	// let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
// }

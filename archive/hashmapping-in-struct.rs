use std::collections::HashMap;

struct Mapping {
	mapping: HashMap<String, String>,
}

fn main() {
	let mut map = Mapping {
		mapping: HashMap::new(),
	};

	map.mapping.insert(
		"Adventures of Huckleberry Finn".to_string(),
		"My favorite book.".to_string(),
	);

	map.mapping.insert(
		"The Adventures of Sherlock Holmes".to_string(),
		"Eye lyked it alot.".to_string(),
	);

	println!("{:?}", map.mapping);
}

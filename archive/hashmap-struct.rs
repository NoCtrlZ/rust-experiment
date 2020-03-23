use std::collections::HashMap;

#[derive(Debug)]
struct Person {
	name: String,
	age: u8
}

struct Mapping {
	mapping: HashMap<String, Person>,
}

fn main() {
	let mut map = Mapping {
		mapping: HashMap::new(),
	};

	map.mapping.insert(
		"number 114514".to_string(),
		Person {
			name: "shinsaku".to_string(),
			age: 23
		},
	);

	println!("{:?}", map.mapping);
	println!("{:?}", map.mapping["number 114514"])
}

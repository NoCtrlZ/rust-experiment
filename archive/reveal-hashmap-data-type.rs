use std::collections::HashMap;

struct Route {
	method: HashMap<String, HashMap<String, String>>,
}

fn main() {
	let mut route = Route {
		method: HashMap::new(),
	};
	route.method.entry("hi".to_string()).or_insert_with(HashMap::new).insert("say".to_string(), "hello".to_string());
	println!("{:?}", route.method["hi"]["say"]);
	println!("{:?}", route.method.get("hi").and_then(|m| m.get("say")).unwrap())
}

use std::collections::HashMap;

fn script_foo(_arg: String) -> String {
    format!("hello world {}", _arg)
}

fn main() {
	let mut map = HashMap::new();
    map.entry("hi").or_insert_with(HashMap::new).insert("greet", script_foo);
	println!("{:?}", map["hi"]["greet"]("shinsaku".to_string()));
}

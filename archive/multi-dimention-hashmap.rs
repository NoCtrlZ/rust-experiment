use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.entry("hi").or_insert_with(HashMap::new).insert(0, 0);

	println!("{:?}", map);
	println!("{:?}", map["hi"][&0])
}
use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub shops: Vec<Shop>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shop {
	id: u32,
	name: String,
	amount: u32
}

fn main() {
 	let mut company = Company {
		shops: Vec::new()
	};
	let shop = Shop {
		id: 8,
		name: "hello".to_string(),
		amount: 5
	};
	company.shops.push(shop);
	let json = json!(company.shops);
	println!("{:?}", company);
	println!("{:?}", json);
}

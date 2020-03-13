struct Company {
	shops: Vec<Shop>
}

#[derive(Debug)]
struct Shop {
	name: String
}

impl Company {
	fn new() -> Company {
		Company {
			shops: Vec::new()
		}
	}

	fn add(&mut self) {
		let shop = Shop {
			name: "shop".to_string()
		};
		self.shops.push(shop);
	}

	fn latest_one(&mut self) -> &mut Shop {
		self.shops.last_mut().unwrap()
    }
}

impl Shop {
	fn change_name(&mut self, name: &str) {
		self.name = name.to_string();
	}
}

fn main() {
	let mut company = Company::new();
	company.add();
	let mut latest_shop = company.latest_one();
	latest_shop.change_name("new shop");
	println!("{:?}", latest_shop);
}

macro_rules! example {
	(x => $e:expr) => (println!("match with x: {}", $e));
	(y => $e:expr) => (println!("match with y: {}", $e));
}

fn main() {
	example!(y => 5);
}
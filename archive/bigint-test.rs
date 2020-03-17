extern crate bigint;
use bigint::U256;

struct Uint256 (
	pub [u64; 4]
);

fn main() {
	let mut val: U256 = 1023.into();
	for _ in 0..200 { val = val * 2.into() }
	println!("{:?}", &val);
	assert_eq!(
		&format!("{}", val),
		"1643897619276947051879427220465009342380213662639797070513307648"
	);
}

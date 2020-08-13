extern crate rand;

use num_bigint::traits::ModInverse;
use num_bigint::RandPrime;
use rand::thread_rng;

fn main() {
	let mut rng = thread_rng();
	let p = rng.gen_prime(1024);

	println!("{:?}", p);
}
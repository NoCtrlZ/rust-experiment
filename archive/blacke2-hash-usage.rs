use blake2_rfc::blake2b::{Blake2b};

fn main() {
	// Using the convenience function.
	let mut hash = Blake2b::with_params(64, &[], &[], b"Zerochain_Master");
	println!("Hash One -> {:?}", hash);
	hash.update(b"The quick brown fox jumps over the lazy dog");
	println!("Hash Two -> {:?}", hash);
	let hashed = hash.finalize();
	println!("Hash Three -> {:?}", hashed);

	let left = &hashed.as_bytes()[..32];
	println!("Hash Left -> {:?}", left);
    let mut right = [0u8; 32];
	println!("Hash Right -> {:?}", right);
    right.copy_from_slice(&hashed.as_bytes()[32..]);
	println!("Hash Right -> {:?}", right);
}
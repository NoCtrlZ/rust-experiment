ARG = name

run: src/main.rs
	cargo run src/main.rs

save: src/main.rs
	cp src/main.rs archive/${ARG}.rs

init: src/main.rs
	echo 'fn main() {\n \tprintln!("hello world");\n}' > src/main.rs
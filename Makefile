ARG = name

run: src/main.rs
	cargo run src/main.rs

save: src/main.rs
	cp src/main.rs archive/${ARG}.rs
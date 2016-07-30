doc:
	cargo build
	rustdoc src/lib.rs --crate-name sgp4 -o ./target/doc -L dependency=./target/debug -L dependency=./target/debug/deps --html-in-header=./docs/header_imports.html

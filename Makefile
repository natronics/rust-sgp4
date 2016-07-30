doc:
	cargo build
	rustdoc src/lib.rs --crate-name sgp4 -o ./target/doc -L dependency=./target/debug -L dependency=./target/debug/deps --html-in-header=./docs/header_imports.html
	echo "<meta http-equiv=refresh content=0;url=sgp4/index.html>" > target/doc/index.html
	cp docs/*.pdf target/doc/

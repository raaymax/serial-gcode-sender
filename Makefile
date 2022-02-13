

default:
	cargo build --release

install: 
	cp ./target/release/laser $$HOME/.local/bin/laser



vape:
	RUSTFLAGS="-Ctarget-cpu=native" cargo build --release
	mv target/release/vape vape
	strip vape

build:
	cargo build --release
	./osx_vst_bundler.sh DigiDist target/release/libdigidist.dylib

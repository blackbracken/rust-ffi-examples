.PHONY: setup
setup:
	cargo install cargo-ndk
	rustup target add \
    	aarch64-linux-android \
    	armv7-linux-androideabi \
    	x86_64-linux-android \
    	i686-linux-android

.PHONY: build_android
build_android:
	cargo ndk -t armeabi-v7a -t arm64-v8a -o ../android/app/src/main/jniLibs build --release

.PHONY: build_wasm
build_wasm:
	wasm-pack build --target bundler

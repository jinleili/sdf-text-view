cd $(pwd)
cargo fmt

cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
# cargo build --target aarch64-linux-android
# cargo build --target armv7-linux-androideabi
# cargo build --target i686-linux-android

cd app/android/app/src/main
rm -rf jniLibs

mkdir jniLibs
mkdir jniLibs/arm64
mkdir jniLibs/armeabi
mkdir jniLibs/x86


ln -s /Users/grenlight/Rust/idroid-rs/target/aarch64-linux-android/release/libidroid.so jniLibs/arm64/libidroid.so
ln -s /Users/grenlight/Rust/idroid-rs/target/armv7-linux-androideabi/release/libidroid.so jniLibs/armeabi/libidroid.so
ln -s /Users/grenlight/Rust/idroid-rs/target/i686-linux-android/release/libidroid.so jniLibs/x86/libidroid.so
# ln -s /Users/grenlight/Rust/idroid2/target/aarch64-linux-android/debug/libidroid.so jniLibs/arm64/libidroid.so
# ln -s /Users/grenlight/Rust/idroid2/target/armv7-linux-androideabi/debug/libidroid.so jniLibs/armeabi/libidroid.so
# ln -s /Users/grenlight/Rust/idroid2/target/i686-linux-android/debug/libidroid.so jniLibs/x86/libidroid.so
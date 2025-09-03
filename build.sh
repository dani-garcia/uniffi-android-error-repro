cross build -p crate-a --release --target=aarch64-linux-android

cargo run -p uniffi-bindgen generate \
  ./target/aarch64-linux-android/release/libcrate_a.so \
  --library \
  --language kotlin \
  --no-format \
  --out-dir ./android/lib/src/main/kotlin


cd android

export ANDROID_HOME=~/Library/Android/sdk
./gradlew build

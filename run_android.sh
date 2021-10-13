set -eux

echo "run debug"
cargo ndk --target armeabi-v7a build
adb push target/armv7-linux-androideabi/debug/experiment_backtrace /data/local/tmp/experiment_backtrace
adb shell 'chmod +x /data/local/tmp/experiment_backtrace && /data/local/tmp/experiment_backtrace'

echo "run release"
cargo ndk --target armeabi-v7a build --release
adb push target/armv7-linux-androideabi/release/experiment_backtrace /data/local/tmp/experiment_backtrace
adb shell 'chmod +x /data/local/tmp/experiment_backtrace && /data/local/tmp/experiment_backtrace'

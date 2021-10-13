set -eux

echo "run debug"
cargo ndk --target armeabi-v7a build
adb push target/armv7-linux-androideabi/debug/experiment_backtrace /data/local/tmp/experiment_backtrace
adb shell 'chmod +x /data/local/tmp/experiment_backtrace && /data/local/tmp/experiment_backtrace'
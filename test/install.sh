sudo rm -f /usr/bin/cmdcreate
cargo update
rustup update
cargo build --release
sudo cp ./target/release/cmdcreate /usr/bin

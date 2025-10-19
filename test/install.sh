sudo pacman -Rns cmdcreate --noconfirm || clear; echo -e "\nRelease ver not installed, skipping\n"
sudo rm -f /usr/bin/cmdcreate
rustup update
cargo update
./format.sh
cargo build --release
cargo clippy
sudo cp ./target/release/cmdcreate /usr/bin

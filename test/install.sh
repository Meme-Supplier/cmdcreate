yay -Rns cmdcreate --noconfirm || clear; echo -e "\nRelease ver not installed, skipping\n"
sudo rm -f /usr/bin/cmdcreate
cargo update
rustup update
./format.sh
cargo build --release
sudo cp ./target/release/cmdcreate /usr/bin

#!/bin/sh

#~ Arch

#& Snap (situationally)
git clone https://aur.archlinux.org/snapd.git
cd snapd
makepkg -si
sudo systemctl enable --now snapd.socket
sudo ln -s /var/lib/snapd/snap /snap

#* Rust
sudo snap install rustup --classic

#* Rust nightly
rustup default nightly
rustup update

#* Diesel
sudo pacman -Syyu gcc-multilib

sudo chown -R $USER:$USER target                                                                                    ✔  20s  
cargo clean
cargo run

cargo install diesel_cli --no-default-features --features postgres    
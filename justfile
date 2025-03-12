install-ubuntu:
    sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
    sudo apt-get install libwayland-dev libxkbcommon-dev
    sudo apt-get install mold clang
    sudo apt-get install git-lfs

rustup:
    rustup component add rustc-codegen-cranelift-preview --toolchain nightly
    rustup component add rust-analyzer

run:
    cargo run

release:
    cargo run --release

pong:
    cargo run --example pong

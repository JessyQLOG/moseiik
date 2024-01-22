FROM rust:latest

ENTRYPOINT [ "cargo", "test", "--release", "--" ]

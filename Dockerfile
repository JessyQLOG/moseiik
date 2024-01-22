FROM rust:latest
WORKDIR /app
ENTRYPOINT [ "cargo", "test", "--release", "--" ]

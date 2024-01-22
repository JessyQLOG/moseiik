FROM rust:latest
WORKDIR /app
RUN git clone https://github.com/JessyQLOG/moseiik.git

RUN cargo build --manifest-path=/app/moseiik/Cargo.toml --release
RUN cargo test --manifest-path=/app/moseiik/Cargo.toml --release --

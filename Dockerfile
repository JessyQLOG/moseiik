FROM rust:latest
WORKDIR /app
RUN git clone https://github.com/JessyQLOG/moseiik.git
RUN cd /app/moseiik
RUN cargo build --manifest-path=/app/moseiik/Cargo.toml --release
RUN cargo --manifest-path=/app/moseiik/Cargo.toml test --release --

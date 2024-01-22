FROM rust:latest
WORKDIR /app
RUN git clone https://github.com/JessyQLOG/moseiik.git

RUN cargo build --release
ENTRYPOINT [ "cargo", "test", "--release", "--" ]

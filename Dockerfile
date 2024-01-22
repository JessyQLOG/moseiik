FROM rust:latest
RUN git clone https://github.com/JessyQLOG/moseiik
ENTRYPOINT [ "cargo", "test", "--release", "--" ]

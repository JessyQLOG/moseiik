FROM rust:latest
RUN git clone https://github.com/JessyQLOG/moseiik && cd moseiik
ENTRYPOINT [ "cargo", "test", "--release", "--" ]

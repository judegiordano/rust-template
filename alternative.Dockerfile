# Rust as the base image
FROM rust:latest

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

RUN rustup default nightly
RUN cargo fetch
RUN cargo install --path .

# Build your program for release
RUN cargo build --release

EXPOSE 3000

# Run the binary
CMD ["./target/release/rust-template"]

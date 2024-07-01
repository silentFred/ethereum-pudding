FROM rust:latest AS builder

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/pudding"]
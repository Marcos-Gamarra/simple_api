# build
FROM rust:latest as builder
LABEL stage=builder
WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
COPY src/ ./src
COPY Cargo.toml .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/simple_api .
EXPOSE 3000
CMD ["./simple_api"]

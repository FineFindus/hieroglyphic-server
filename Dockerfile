FROM rust:1.95.0-alpine3.22 as builder
WORKDIR /build
COPY . .
ENV RUSTFLAGS "-C target-cpu=native"
RUN cargo build --release
RUN mv /build/target/release/hieroglyphic-server /build/app.bin

FROM docker.io/alpine:3
RUN mkdir /app
WORKDIR /app
COPY --from=builder /build/app.bin /app/app.bin

ENV RUST_LOG=info
CMD ["/app/app.bin"]

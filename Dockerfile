FROM rust:latest as builder
WORKDIR /usr/src/metrics-filter
COPY . .
RUN cargo build --release

FROM debian:latest
COPY --from=builder /usr/src/metrics-filter/target/release/metrics-filter /usr/local/bin/
RUN apt update && apt install openssl -y
CMD ["/usr/local/bin/metrics-filter"]

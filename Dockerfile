FROM rust:1.52 as builder

WORKDIR /usr/src/backend-rs
COPY . .
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/backend-rs/target/release/backend-rs /usr/local/bin/backend-rs

EXPOSE 8080

CMD ["backend-rs"]

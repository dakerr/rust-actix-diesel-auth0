FROM rust:1.52

WORKDIR /usr/src/backend-rs
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./backend-rs /bin/
EXPOSE 8080
ENTRYPOINT ["/bin/backend-rs"]

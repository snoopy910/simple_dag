# syntax=docker/dockerfile:1.4
FROM rust:latest

WORKDIR /usr/src/app
RUN cargo init
COPY Cargo.toml Cargo.lock database.txt ./
RUN cargo build --release
COPY src ./src
RUN cargo build --release


EXPOSE 8000

CMD [ "cargo", "run" ]

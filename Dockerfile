# TODO: Can experiment using other images...
#		These notes were from another project I had

# Alpine does not work for some of these libraries =(
# FROM rust:1.70.0-alpine3.18

# Debian does not work with distroless =(
# FROM rust:1.70.0-slim-bookworm as build

# Build stage
FROM clux/muslrust:1.72.0-nightly-2023-06-16 as build

WORKDIR /usr/src/app

RUN cargo init

# FIXME: Might not be required for the nightly image I am using now...
RUN rustup default nightly

COPY Cargo.lock Cargo.toml .

RUN cargo build --release

COPY src src

RUN cargo build --release --features=derive

# Application stage
FROM gcr.io/distroless/static-debian11 as app

COPY --from=build /usr/src/app/target/x86_64-unknown-linux-musl/release/walrus /

# Unsure what needs to be exposed / ENV Vars set
# TODO: Will need to expose a port in order to get liveness / readiness endpoints up and running correctly
# EXPOSE 8000
# ENV SERVER_HOST "0.0.0.0"

CMD ["./walrus"]

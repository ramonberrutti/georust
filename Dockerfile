FROM rust:1.43-alpine AS build

WORKDIR /usr/src/georust

# Rocket only compiled with nightly version of rust.
RUN rustup override set nightly

# For ring crate.
RUN apk add --no-cache musl-dev

# Download dependencies and compile to reuse the cache.
# Waiting a pretty solution: https://github.com/rust-lang/cargo/issues/2644
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release || true

COPY ./src ./src

RUN cargo build --release

FROM alpine

COPY --from=build /usr/src/georust/target/release/georust /usr/local/bin

EXPOSE 8000
ENTRYPOINT ["georust"]

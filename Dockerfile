# Dockerfile for Rust Boilerplate
#
# Inspired by:
# - https://depot.dev/blog/rust-dockerfile-best-practices
# - https://github.com/LukeMathWalker/cargo-chef

FROM rust:1.81 AS base
WORKDIR /app

RUN apt-get update && \
    apt-get install musl-tools -y && \
		rustup target add x86_64-unknown-linux-musl && \
    # Install cargo
    cargo install cargo-binstall && \
    cargo binstall cargo-chef -y && \
		cargo binstall sccache -y

ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

FROM base AS planner

COPY Cargo.toml ./
COPY ./src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

FROM base AS builder

COPY --from=planner /app/recipe.json .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release --target x86_64-unknown-linux-musl

FROM scratch AS runtime
WORKDIR /app

COPY config.example.yml ./
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-boilerplate /usr/local/bin/app

ENTRYPOINT ["/usr/local/bin/app"]
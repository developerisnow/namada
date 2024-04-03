FROM lukemathwalker/cargo-chef:rust-1.76.0-bullseye AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app

# Pin versions for all packages and perform a clean-up after installation.
RUN apt-get update && apt-get install -y \
    build-essential=12.9 \
    clang-tools-11=1:11.0.1-2 \
    git=1:2.30.2-1 \
    libssl-dev=1.1.1n-0+deb11u3 \
    pkg-config=0.29.2-1 \
    protobuf-compiler=3.12.4-1 \
    libudev-dev=247.3-6+deb11u1 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN make build-release

FROM golang:1.18.0 as tendermint-builder
WORKDIR /app
RUN git clone -b v0.37.2 --single-branch https://github.com/cometbft/cometbft.git && cd cometbft && make build

# Use 'debian:bullseye-slim' version for consistency.
FROM debian:bullseye-slim AS runtime
ENV NAMADA_LOG_COLOR=false

# Reduce layer count by combining commands.
RUN apt-get update && apt-get install -y libcurl4-openssl-dev libudev-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home namada

USER namada

COPY --from=builder --chmod=0755 /app/target/release/namada /usr/local/bin
COPY --from=builder --chmod=0755 /app/target/release/namadan /usr/local/bin
COPY --from=builder --chmod=0755 /app/target/release/namadaw /usr/local/bin
COPY --from=builder --chmod=0755 /app/target/release/namadac /usr/local/bin
COPY --from=tendermint-builder --chmod=0755 /app/cometbft/build/cometbft /usr/local/bin

EXPOSE 26656 26660 26659 26657

ENTRYPOINT ["/usr/local/bin/namada"]
CMD ["--help"]

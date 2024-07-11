FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /pim
LABEL authors="jakob753951"

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /pim/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin pim

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm AS runtime
WORKDIR /pim
COPY --from=builder /pim/target/release/pim /usr/local/bin
ENTRYPOINT ["/usr/local/bin/pim"]

# Base image for Cargo Chef
FROM kebtech/rust-cargo-chef-debian:1.81.0 as chef
WORKDIR /app

# Copy the application and dependencies
COPY ./local_dependencies ./local_dependencies
COPY ./local_dependencies/Cargo.toml.modified ./Cargo.toml

# Stage 1: Generate the recipe.json
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Cache dependencies
FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 3: Build the application
FROM chef AS builder
COPY . .
COPY --from=cacher /app/target /app/target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# Final stage: Runtime image
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update \
    && apt-get install -y ca-certificates libc6 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/blockchain-tools-server ./blockchain-tools-server
ENTRYPOINT ["./blockchain-tools-server"]

# Base image specifying the platform
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Stage 1: Prepare the recipe.json using the actual source code
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Cache dependencies using cargo-chef
FROM chef as cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 3: Build the actual application using the cached dependencies
FROM chef as builder
COPY . .
# Reuse compiled dependencies cached in the previous stage
COPY --from=cacher /app/target /app/target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# Final stage: Create the runtime image
FROM debian as runtime
WORKDIR /app
# Only copy the built binary and necessary libraries
COPY --from=builder /app/target/release/blockchain-tools-server ./blockchain-tools-server
RUN apt-get update \
    && apt-get install -y ca-certificates libc6 \
    && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["./blockchain-tools-server"]

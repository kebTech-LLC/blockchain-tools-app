# Base image specifying the platform
FROM kebtech/rust-cargo-chef-debian:1.81.0 as base
WORKDIR /app

# Stage 1: Prepare the recipe.json using the actual source code
FROM base as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Cache dependencies using cargo-chef
FROM base as cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 3: Build the actual application using the cached dependencies
FROM base as builder
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

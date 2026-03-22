# Build stage
FROM rust:1.76-slim as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Create dummy main.rs for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Download dependencies (cached layer)
RUN cargo build --release && rm -rf target/release/deps/rust_image_api*

# Copy actual source
COPY src/main.rs src/

# Build
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/rust-image-api .

# Expose port
EXPOSE 8080

# Run
CMD ["./rust-image-api"]

# Revolution Trading Pros - Rust API
# ICT 11+ Principal Engineer Grade
# Multi-stage optimized Docker build

# =============================================================================
# Stage 1: Build
# =============================================================================
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project for dependency caching
RUN USER=root cargo new --bin revolution-trading-pros-api
WORKDIR /app/revolution-trading-pros-api

# Copy manifests for dependency caching
COPY Cargo.toml Cargo.lock ./

# Build dependencies only (this layer will be cached)
RUN cargo build --release && rm -rf src

# Copy source code
COPY src ./src

# Touch main.rs to trigger rebuild of our code (not dependencies)
RUN touch src/main.rs

# Build the application
RUN cargo build --release --locked

# =============================================================================
# Stage 2: Runtime
# =============================================================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 -s /bin/bash appuser

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/revolution-trading-pros-api/target/release/revolution-trading-pros-api /app/

# Set ownership
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Environment variables
ENV RUST_LOG=info,sqlx=warn
ENV HOST=0.0.0.0
ENV PORT=8080

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/health/live || exit 1

# Run the application
CMD ["./revolution-trading-pros-api"]

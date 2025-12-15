# Revolution Trading Pros - Rust API
# Multi-stage build for minimal image size

# Build stage - Rust 1.92 (stable December 2025)
FROM rust:1.92-alpine3.21 AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig perl make

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create dummy main.rs to cache dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build dependencies (cached layer)
RUN cargo build --release && rm -rf src

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build the actual application
RUN touch src/main.rs && cargo build --release

# Runtime stage - Alpine 3.21 (December 2025)
FROM alpine:3.21 AS runtime

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Create non-root user
RUN addgroup -g 1000 app && adduser -u 1000 -G app -h /app -D app

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/revolution-api ./
COPY --from=builder /app/migrations ./migrations

# Set ownership
RUN chown -R app:app /app

USER app

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget -q --spider http://localhost:8080/health || exit 1

# Run
ENV RUST_LOG=info
CMD ["./revolution-api"]

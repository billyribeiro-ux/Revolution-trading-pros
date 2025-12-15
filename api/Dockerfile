# Revolution Trading Pros - Rust API
# Multi-stage build for minimal image size

# Build stage - Rust 1.84 (December 2025)
FROM rust:1.84-alpine3.21 AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig perl make

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create dummy main.rs to cache dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Generate lock file and pin ALL problematic crates to Rust 1.84 compatible versions
# Many AWS SDK crates released after Nov 2024 require Rust 1.88 (edition2024)
RUN cargo generate-lockfile && \
    cargo update -p home --precise 0.5.9 && \
    cargo update -p base64ct --precise 1.6.0 && \
    cargo update -p aws-config --precise 1.1.8 && \
    cargo update -p aws-sdk-s3 --precise 1.21.0 && \
    cargo update -p aws-sdk-sso --precise 1.15.0 && \
    cargo update -p aws-sdk-ssooidc --precise 1.15.0 && \
    cargo update -p aws-sdk-sts --precise 1.15.0 && \
    cargo update -p aws-credential-types --precise 1.1.8 && \
    cargo update -p aws-runtime --precise 1.1.8 && \
    cargo update -p aws-sigv4 --precise 1.1.8 && \
    cargo update -p aws-smithy-async --precise 1.1.8 && \
    cargo update -p aws-smithy-eventstream --precise 0.60.4 && \
    cargo update -p aws-smithy-http --precise 0.60.8 && \
    cargo update -p aws-smithy-json --precise 0.60.7 && \
    cargo update -p aws-smithy-query --precise 0.60.7 && \
    cargo update -p aws-smithy-runtime --precise 1.1.8 && \
    cargo update -p aws-smithy-runtime-api --precise 1.1.8 && \
    cargo update -p aws-smithy-types --precise 1.1.8 && \
    cargo update -p aws-smithy-xml --precise 0.60.8 && \
    cargo update -p aws-types --precise 1.1.8

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

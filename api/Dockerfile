# Revolution Trading Pros - Rust API
# Multi-stage build for minimal image size

# FIX-2026-04-26: pinned from rust:latest to explicit version (Phase 7.3)
# FROM rust:latest AS builder
# FIX-2026-06-08: keep Docker builder aligned with rust-toolchain.toml.
# 2026-10-09: Rust 1.99, and both stages moved to Debian 13 "trixie" — the
# current Debian stable (bookworm is now oldstable). Builder and runtime must
# stay on the same Debian release so the binary's glibc/OpenSSL ABI matches.
# 2026-10-28: Rust 1.99.1-trixie.
FROM rust:1.99.1-trixie AS builder

# Install build dependencies (Debian-based image)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev perl make \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create dummy main.rs to cache dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build dependencies (cached layer)
RUN cargo build --release && rm -rf src

# Copy source code and SQLx offline cache
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

# Build the actual application with SQLx offline mode
ENV SQLX_OFFLINE=true
RUN touch src/main.rs && cargo build --release

# FIX-2026-04-26 moved this stage to bookworm-slim while trixie was still
# Debian testing. Trixie has since become Debian 13 stable, so on 2026-10-09
# the runtime returned to trixie-slim to match the rust:1.99.0-trixie builder.
FROM debian:trixie-slim AS runtime

# Install runtime dependencies (OpenSSL, CA certs, curl for health checks).
# On trixie the OpenSSL 3 runtime package is `libssl3t64` (64-bit time_t
# transition); `libssl3` is only a virtual name it Provides. Named explicitly
# so the binary's runtime dependency (sqlx/redis native-tls) is documented.
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libssl3t64 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -g 1000 app && useradd -u 1000 -g app -d /app -m app

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/revolution-api ./
COPY --from=builder /app/migrations ./migrations

# Set ownership
RUN chown -R app:app /app

USER app

# Expose port
EXPOSE 8080

# Health check (using curl on Debian)
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run
ENV RUST_LOG=info
CMD ["./revolution-api"]

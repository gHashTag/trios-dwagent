<<<<<<< HEAD
FROM rust:1.82-bookworm AS builder
=======
# Dockerfile for trios-dwagent on Railway (RustDesk Server) v2
FROM rust:1.88-bookworm as builder
>>>>>>> a111b82 (chore: local backup 2026-06-17)

WORKDIR /app
COPY . .
RUN cargo build --release -p igla-trainer

<<<<<<< HEAD
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates git && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/igla-trainer /usr/local/bin/
ENTRYPOINT ["igla-trainer"]
=======
# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifest and lock
COPY Cargo.toml Cargo.lock* ./

# Copy actual source
COPY src ./src

# Build
RUN cargo build --release && \
    strip /app/target/release/trios-dwagent

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/trios-dwagent /app/trios-dwagent

# Make executable
RUN chmod +x /app/trios-dwagent

# Create RustDesk Server directory
RUN mkdir -p /app/rustdesk-server

# Expose RustDesk Server ports
EXPOSE 21114 21115 21116 21117 21118 21119

# Default command - start RustDesk Server
ENTRYPOINT ["/app/trios-dwagent"]
CMD ["setup"]
>>>>>>> a111b82 (chore: local backup 2026-06-17)

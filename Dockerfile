# Dockerfile for trios-dwagent on Railway (RustDesk Server) v2
FROM rust:1.88-bookworm as builder

WORKDIR /app

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

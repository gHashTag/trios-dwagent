# trios-dwagent

> RustDesk Server installer for Railway deployment

A lightweight Rust CLI utility for deploying RustDesk Server (self-hosted remote desktop) to Railway containers. RustDesk is a fully open-source remote desktop solution written in Rust.

## Features

- **Pure Rust** implementation - no shell scripts or Python
- **Automatic binary download** from RustDesk GitHub releases
- **Railway-ready** Dockerfile with multi-stage build
- **GitHub Actions** workflow for automatic deployments
- **Clippy-clean**: Zero warnings, production-ready code

## What is RustDesk Server?

RustDesk Server consists of two main components:
- **hbbs** - Rendezvous/ID server (handles connections and NAT traversal)
- **hbbr** - Relay server (for direct P2P connections)

Both are written in pure Rust and compile to small, efficient binaries.

## Installation

### Local Build

```bash
# Build from trios repository
cd /Users/playra/trios
cargo build -p trios-dwagent --release

# The binary will be at target/release/trios-dwagent
```

### Cross-compile for Linux

```bash
# Add Linux target
rustup target add x86_64-unknown-linux-gnu

# Build for Linux deployment
cargo build -p trios-dwagent --release --target x86_64-unknown-linux-gnu
```

## Usage

```bash
# Full setup (download + start)
trios-dwagent setup

# Download binaries only
trios-dwagent download

# Force re-download
trios-dwagent download --force

# Start servers
trios-dwagent start

# Restart servers
trios-dwagent start --restart

# Check status
trios-dwagent status

# Stop servers
trios-dwagent stop

# Clean up downloaded files
trios-dwagent cleanup

# Display help
trios-dwagent --help
```

## Deployment

### Railway Setup

```bash
# Link to existing project
railway link -p e4fe33bb-3b09-4842-9782-7d2dea1abc9b

# Deploy
railway up

# Or build and deploy from Dockerfile
railway deploy
```

### Railway Shell (manual testing)

```bash
# Open shell
railway shell

# Run setup
./trios-dwagent setup

# Check status
./trios-dwagent status
```

## Configuration

### Railway Config

Railway auto-detects `railway.toml` in the crate root:
- Uses `rust:slim` (latest) for optimal build
- Deploys to project IGLA
- Memory: 256MB, CPU: 0.5 vCPU
- Restart on failure (max 3 retries)

### Exposed Ports

The Dockerfile exposes the following RustDesk Server ports:

| Port | Service | Description |
|------|---------|-------------|
| 21114 | Web | Web client (optional) |
| 21115 | HBBS | ID/Rendezvous server |
| 21116 | HBBR | Relay server |
| 21117 | API | Web API (optional) |
| 21118/21119 | Additional | Reserved for future use |

## Connecting with RustDesk Client

1. Download [RustDesk Client](https://rustdesk.com/)
2. Configure the connection settings:
   - **ID Server**: `<your-railway-host>:21115`
   - **Relay**: `<your-railway-host>:21116`
3. Your server will appear in the machine list

### Finding Your Railway Host

```bash
railway domains
# Or check Railway dashboard for the deployment URL
```

## Architecture

```
┌─────────────┐     ┌──────────────────┐     ┌─────────────┐
│   Client    │────▶│  hbbs (ID/Port)  │────▶│   Client    │
│  (RustDesk) │     │    Port: 21115   │     │  (RustDesk) │
└─────────────┘     └──────────────────┘     └─────────────┘
                          │
                          ▼
                    ┌─────────────┐
                    │  hbbr       │
                    │  (Relay)    │
                    │  Port: 21116│
                    └─────────────┘
```

## Development

### Build and Test

```bash
# Debug build
cargo build -p trios-dwagent

# Release build
cargo build -p trios-dwagent --release

# Run tests
cargo test -p trios-dwagent

# Lint (must pass before merge)
cargo clippy -p trios-dwagent -- -D warnings

# Format
cargo fmt -p trios-dwagent
```

## Links

- [Trios Repository](https://github.com/gHashTag/trios)
- [RustDesk](https://rustdesk.com/)
- [RustDesk Server GitHub](https://github.com/rustdesk/rustdesk-server)
- [Railway](https://railway.app)

# trios 🔱

<<<<<<< HEAD
> **Trinity Git Orchestrator** — Dual-MCP + Vision bridge for AI agents to control Git & GitButler through BrowserOS

[![CI](https://github.com/gHashTag/trios/actions/workflows/ci.yml/badge.svg)](https://github.com/gHashTag/trios/actions)
=======
> RustDesk Server installer for Railway deployment

A lightweight Rust CLI utility for deploying RustDesk Server (self-hosted remote desktop) to Railway containers. RustDesk is a fully open-source remote desktop solution written in Rust.
>>>>>>> a111b82 (chore: local backup 2026-06-17)

## What is trios?

<<<<<<< HEAD
`trios` is a **dual-layer MCP system** that allows AI agents (BrowserOS, Claude Code, Cursor) to control Git repositories and GitButler virtual branches through **natural language + vision**.

### Two Layers
=======
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
>>>>>>> a111b82 (chore: local backup 2026-06-17)

| Layer | Stack | Port | Purpose |
|-------|-------|------|---------|
| **trios-server** (Rust) | Axum + git2-rs + but CLI | `9005` | Core git operations, stable & fast |
| **trios-mcp-bridge** (TypeScript) | Bun + Hono + MCP SDK | `9200` | Vision + high-level GitButler workflows |

The agent can use either or both:
- **Rust server** for pure git operations (no UI needed)
- **TypeScript bridge** for vision-enhanced workflows (sees GitButler UI, understands context)

## Architecture

```
╔══════════════════════════════════════════════════════════════╗
║                 TRIOS DUAL-MCP + VISION                      ║
╚══════════════════════════════════════════════════════════════╝

                  ┌────────────────────────────────────┐
                  │   BrowserOS (Chromium fork)        │
                  │   + Agent Extension (React + Vision)│
                  └──────────────────┬─────────────────┘
                                     │
                          MCP + Vision (screenshots, DOM)
                                     ▼
        ┌───────────────────────────────────────────────────┐
        │           trios-mcp-bridge (Bun/Hono) :9200       │
        │   • MCP client → BrowserOS MCP (screenshots)      │
        │   • MCP client → GitButler MCP (`but mcp`)        │
        │   • 10 high-level tools:                          │
        │     gitbutler_analyze_ui()                        │
        │     gitbutler_commit_visible()                    │
        │     gitbutler_create_branch()                     │
        │     gitbutler_push_stack()                        │
        └──────────────────┬────────────────────────────────┘
                           │
       ┌───────────────────┼───────────────────────┐
       │                   │                       │
┌──────▼──────┐  ┌─────────▼──────────┐  ┌────────▼─────────┐
│ BrowserOS   │  │ trios-server       │  │ GitButler MCP    │
│ MCP Server  │  │ (Rust/Axum) :9005  │  │ (`but mcp`)      │
│ (CDP)       │  │ git2-rs + but CLI  │  │ virtual branches │
│ screenshots │  │ stage, commit,     │  │ stacks, absorb   │
│ snapshots   │  │ branch, push, pull │  │ undo             │
└─────────────┘  └────────────────────┘  └──────────────────┘
                           │                       │
                           └───────┬───────────────┘
                                   ▼
                              .git/ + GitButler state
                              (UI updates automatically)
```

## Quick Start

### Rust Server (core git ops)

```bash
# Clone
git clone https://github.com/gHashTag/trios
cd trios

# Build & Run
cargo build
cargo run -p trios-server
# Server starts at http://localhost:9005

# Test
cargo test
```

### TypeScript Bridge (vision + workflows)

```bash
<<<<<<< HEAD
# Located at: packages/browseros-agent/apps/trios-mcp-bridge/
cd packages/browseros-agent/apps/trios-mcp-bridge

# Install & Run
bun install
bun run src/index.ts
# Bridge starts at http://localhost:9200

# With options
bun run src/index.ts --port 9200 --browseros-url http://127.0.0.1:9105/mcp --working-dir /path/to/repo
=======
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
>>>>>>> a111b82 (chore: local backup 2026-06-17)
```

## MCP Tools

<<<<<<< HEAD
### Rust Server (trios-server :9005) — 7 Core Tools

| Tool | Crate | Description |
|------|-------|-------------|
| `git_status` | trios-git | List changed files |
| `git_stage_files` | trios-git | Stage files for commit |
| `git_commit` | trios-git | Create a commit |
| `git_branch_list` | trios-git | List branches |
| `git_branch_create` | trios-git | Create a branch |
| `git_push` | trios-git | Push to remote |
| `git_pull` | trios-git | Pull from remote |

### TypeScript Bridge (trios-mcp-bridge :9200) — 10 Vision + Workflow Tools

#### Vision & Analysis
| Tool | Description |
|------|-------------|
| `gitbutler_analyze_ui` | Screenshot GitButler UI + analyze state (branch, files, stacks) |
| `gitbutler_screenshot` | Raw screenshot of GitButler tab |
| `gitbutler_workspace_status` | Detailed file/branch status from CLI |
| `gitbutler_bridge_health` | Health check for all connections |

#### Git Operations
| Tool | Description |
|------|-------------|
| `gitbutler_commit_visible` | Commit changed files with a message |
| `gitbutler_create_branch` | Create a new virtual branch |
| `gitbutler_push_stack` | Push current stack/branch to remote |
| `gitbutler_stage` | Stage specific files |
| `gitbutler_absorb` | Smart absorb changes into appropriate commits |
| `gitbutler_pull` | Pull latest changes |

## MCP API Examples

### Rust Server (port 9005)
=======
### Railway Setup
>>>>>>> a111b82 (chore: local backup 2026-06-17)

```bash
# Stage files
curl -X POST http://localhost:9005/mcp/tools/call \
  -H 'Content-Type: application/json' \
  -d '{"name": "git_stage_files", "input": {"repo_path": "/path/to/repo", "paths": ["src/main.rs"]}}'

<<<<<<< HEAD
# Commit
curl -X POST http://localhost:9005/mcp/tools/call \
  -H 'Content-Type: application/json' \
  -d '{"name": "git_commit", "input": {"repo_path": "/path/to/repo", "message": "feat: add feature"}}'
```

### TypeScript Bridge (port 9200)

```bash
# Health check
curl http://localhost:9200/

# MCP tools via Streamable HTTP
curl -X POST http://localhost:9200/mcp \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","method":"tools/list","id":1}'
=======
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
>>>>>>> a111b82 (chore: local backup 2026-06-17)
```

## Integration

### With BrowserOS

<<<<<<< HEAD
Add as a custom MCP server in BrowserOS settings:

```json
{
  "name": "trios-mcp-bridge",
  "url": "http://127.0.0.1:9200/mcp",
  "transport": "streamable-http"
}
=======
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
>>>>>>> a111b82 (chore: local backup 2026-06-17)
```

### With Claude Code / Cursor

<<<<<<< HEAD
```json
{
  "mcpServers": {
    "trios-bridge": {
      "url": "http://127.0.0.1:9200/mcp",
      "transport": "streamable-http"
    },
    "trios-server": {
      "url": "http://127.0.0.1:9005/mcp",
      "transport": "streamable-http"
    }
  }
}
```

## Example Workflow

```
User: "See what's changed in GitButler and commit the auth changes"

Agent:
1. gitbutler_analyze_ui()     → Sees: branch "feature/auth", 3 changed files
2. gitbutler_stage(["auth.ts", "auth.test.ts"])
3. gitbutler_commit_visible("feat: add auth validation")
4. gitbutler_push_stack()
```

## Crates (Rust)

| Crate | Purpose |
|-------|--------|
| `trios-core` | Traits, types, shared abstractions |
| `trios-git` | Git operations via libgit2 |
| `trios-gb` | GitButler CLI wrapper with fallback |
| `trios-server` | Axum MCP HTTP server on port 9005 |

## Project Structure

```
trios/                              # Rust MCP server (this repo)
├── crates/
│   ├── trios-core/                 # Shared types & traits
│   ├── trios-git/                  # git2-rs operations
│   ├── trios-gb/                   # GitButler CLI wrapper
│   └── trios-server/               # Axum MCP server :9005
├── CLAUDE.md                       # Development laws
└── README.md                       # This file

BrowserOS/packages/browseros-agent/apps/trios-mcp-bridge/   # TypeScript bridge
├── src/
│   ├── index.ts                    # Main entry — Hono HTTP server :9200
│   ├── config.ts                   # Config from env vars + CLI args
│   ├── types.ts                    # Shared types
│   ├── bridge-server.ts            # MCP server with 10 tools
│   └── clients/
│       ├── browseros-client.ts     # BrowserOS MCP client (HTTP)
│       └── gitbutler-client.ts     # GitButler MCP client (stdio) + CLI fallback
├── package.json
└── README.md
```

## Laws

See [CLAUDE.md](./CLAUDE.md) for full rules. Summary:
- NO `.sh` files
- `cargo clippy -- -D warnings` = 0
- `cargo test` before merge
- Every PR closes an issue

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `TRIONS_BRIDGE_PORT` | `9200` | Bridge server port |
| `TRIONS_BROWSEROS_MCP_URL` | `http://127.0.0.1:9105/mcp` | BrowserOS MCP URL |
| `TRIONS_GITBUTLER_CLI` | `but` | GitButler CLI path |
| `TRIONS_GITBUTLER_INTERNAL` | `true` | Use GitButler internal MCP tools |
| `TRIONS_WORKING_DIR` | `cwd` | Working directory for git |
| `TRIONS_LOG_LEVEL` | `info` | Log level |

## Related

- [gHashTag/t27](https://github.com/gHashTag/t27) — Trinity math research
- [gHashTag/BrowserOS](https://github.com/gHashTag/BrowserOS) — Agent that uses trios
- [gHashTag/gitbutler](https://github.com/gHashTag/gitbutler) — GitButler fork
=======
- [Trios Repository](https://github.com/gHashTag/trios)
- [RustDesk](https://rustdesk.com/)
- [RustDesk Server GitHub](https://github.com/rustdesk/rustdesk-server)
- [Railway](https://railway.app)
>>>>>>> a111b82 (chore: local backup 2026-06-17)

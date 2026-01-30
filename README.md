# guildsync

Rust CLI scaffold + specification for synchronizing Discord guild state across:

- Discord guild / guild dump / upload formats (versioned JSON)
- Terminal-run workflows (OpenCode, Codex, interpreters, tmux, MCP servers)
- On-demand Kubernetes (local cluster on your computer; remote cluster test/deploy)
- Remote SSH computers (including hosts reachable only via VPN)

This repository intentionally provides a coherent CLI surface and documentation first.
Most commands currently print `scaffold only; not implemented`.

## Concepts

- `guild dump format`: a machine-readable JSON snapshot of guild structure/state suitable for sync/versioning.
- `upload format`: a machine-readable JSON plan/intended state suitable for applying to a target guild.
- `terminal target`: a local or remote work environment (tmux + OpenCode/Codex + interpreters/MCP) that consumes or produces artifacts.

## Install (dev)

```bash
make install_rust_toolchain

cd terminal-translate-discord-guild
. "$HOME/.cargo/env"
cargo build
```


## Quickstart (scaffold)

```bash
. "$HOME/.cargo/env"

# Export a guild to dump JSON
cargo run -- discord export --guild 123 --out guild.dump.json

# Validate a dump/upload file
cargo run -- format validate --in guild.dump.json --format dump

# Import into a guild (dry-run)
cargo run -- discord import --in guild.dump.json --guild 123 --dry-run

# Local on-demand Kubernetes (documented intent)
cargo run -- kube local status

# Remote test/deploy by kube context (documented intent)
cargo run -- kube remote test --context dev

# SSH command execution (documented intent)
cargo run -- ssh exec --host mybox -- uname -a
```

## Command surface

- `guildsync discord export --guild <ID> --out <PATH>`
- `guildsync discord import --in <PATH> --guild <ID> [--dry-run]`
- `guildsync format validate --in <PATH> [--format dump|upload]`
- `guildsync terminal opencode attach [--tmux <SESSION>]`
- `guildsync kube local up|down|status`
- `guildsync kube remote test|deploy --context <KUBE_CONTEXT>`
- `guildsync ssh exec --host <HOST> -- <CMD...>`

Global flags:
- `--config <PATH>`: override config path
- `--json`: JSON output (best-effort)
- `--log error|warn|info|debug|trace`

## Configuration

Recommended location:
- Linux: `~/.config/guildsync/config.toml`

Recommended secret handling policy:
- Discord bot token should come from an environment variable (not stored in plaintext in config).

Example `config.toml`:

```toml
[discord]
token_env = "DISCORD_TOKEN"

[formats]
dump_version = 1
upload_version = 1
strict = true

[terminal]
tmux_default_session = "opencode"

[kube.local]
provider = "kind" # documented intent; not implemented

[kube.remote]
contexts = ["dev", "staging"]

[ssh]
user = "stc"
identity_file = "~/.ssh/id_ed25519"
known_hosts_mode = "strict"
```

## Security and policy notes

- Discord: operate only on guilds you admin; respect rate limits; avoid logging message content or tokens.
- Tokens: prefer env vars; redact secrets from logs; never commit tokens.
- Kubernetes: use kubeconfig contexts; respect RBAC; do not copy cluster credentials into dumps.
- SSH: key-based auth; strict host key checking by default; be explicit about VPN requirements.

## Non-goals (for this scaffold)

- Implementing full Discord export/import logic
- Running real Kubernetes cluster operations
- Executing real SSH sessions
- Shipping CI/CD, Helm charts, or production deployment automation

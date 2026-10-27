# Jackup

Rust-based infrastructure orchestration for cloud repatriation, self-hosting,
and zero-downtime migration.

> **Status:** Foundation phase — core architecture and security primitives are
> in place. Production deployment, migration providers, and UI integrations are
> being developed incrementally.

## What Jackup provides

- A shared domain model for the CLI, agent, and application surfaces.
- A headless target-server agent with a small, platform-neutral composition
  root.
- Secure local vault primitives using Argon2id and
  ChaCha20-Poly1305 authenticated encryption.
- Versioned control messages with transport and storage seams kept behind
  explicit interfaces.
- A workspace structure designed for testability, clear dependency direction,
  and future desktop/web adapters.

## Workspace

| Crate | Responsibility |
| --- | --- |
| `jackup-core` | Domain models and invariants |
| `jackup-config` | Runtime configuration |
| `jackup-crypto` | Key derivation and authenticated encryption |
| `jackup-protocol` | Versioned control messages |
| `jackup-transport` | Transport adapters |
| `jackup-storage` | Persistence adapters |
| `jackup-backup` | Backup pipeline interfaces |
| `jackup-orchestrator` | Application use cases and policy |
| `jackup-agent` | Target-server agent |
| `jackup-cli` | Command-line interface |
| `jackup-app` | Desktop/web application state |

See [`docs/architecture.md`](docs/architecture.md) for dependency direction,
module responsibilities, and composition roots.

## Development

### Requirements

- Rust stable toolchain
- Cargo

### Common commands

```bash
# Run all tests
cargo test --workspace

# Check compilation
cargo check --workspace --all-targets

# Run formatting and lint checks
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# Run the initial CLI command
cargo run -p jackup-cli -- status
```

## License

Jackup is released under the **MIT License**, a free and permissive
open-source license. No paid license, subscription, or registration is
required to use, modify, or distribute this repository.

The MIT License permits commercial and non-commercial use, provided that the
copyright notice and license text are retained. See
[`LICENSE`](LICENSE) for the complete terms.

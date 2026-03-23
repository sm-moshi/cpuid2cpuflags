# CLAUDE — Runtime Instructions

This is a Rust CLI and library crate that detects CPU capabilities and outputs
Gentoo `CPU_FLAGS_*` values. Hard fork of `gentoo/cpuid2cpuflags`, rewritten in Rust.

**Authoritative policy lives in [AGENTS.md](AGENTS.md).**
If this file conflicts with `AGENTS.md`, follow `AGENTS.md`.

## Commands

```bash
# Development (via mise)
mise run test              # Run tests via cargo-nextest
mise run lint              # Clippy with pedantic warnings as errors
mise run fmt               # Format check (nightly rustfmt)
mise run fmt:fix           # Auto-format
mise run audit             # cargo-audit for known vulnerabilities
mise run deny              # cargo-deny for licence/advisory checks
mise run docs              # Build docs with -D warnings
mise run udeps             # Check for unused deps (nightly)
mise run compat            # Run upstream compatibility tests only

# Raw cargo equivalents
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo +nightly fmt --all --check
cargo +nightly fmt --all
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

## Feature Flags

Dependencies are extensively feature-gated. `default = ["cli", "cflags"]`.

| Feature | What it enables | Key deps |
|---------|----------------|----------|
| `cli` | Command-line argument parsing | clap, anyhow |
| `cflags` | CFLAGS-to-CPU_FLAGS conversion (issue #24) | none |
| `serde` | Serialisation of `FlagSet` | serde |
| `riscv` | RISC-V detection (staging — Gentoo flag set not stabilised) | none |
| `dump` | Dump subcommand for generating test data | none |

When adding new dependencies, make them `optional = true` and gate behind a feature
unless they are core CPU detection.

## Architecture

The crate is split into a library (`lib.rs`) and a binary (`main.rs`).

**Core flow**: `detect::*::detect()` → `FlagSet` → `Display` output

- `tables/*.rs` — Flag definition tables (always compiled, all architectures)
- `detect/*.rs` — Platform-specific detection backends (`#[cfg(target_arch)]`)
- `detect/mock.rs` — Mock backend for testing (parses upstream test data format)
- `cflags/*.rs` — CFLAGS → CPU_FLAGS conversion (feature-gated)
- `flags.rs` — `FlagSet` type with sorted, deduplicated flag output
- `error.rs` — `thiserror`-based error enum with feature-gated variants

### Compatibility Contract

The output of `cpuid2cpuflags` on a given CPU MUST match the output of the
upstream C `gentoo/cpuid2cpuflags` for the same CPU. The test suite enforces
this using the upstream project's test data files verbatim.

Flag tables MUST match Gentoo `profiles/desc/cpu_flags_*.desc` definitions.

## Code Conventions

- **Synchronous only** — no async, no concurrency
- **No unsafe** — `deny` via `[lints.rust]`, single exception in hwcap module
- **Error handling**: `thiserror` for library errors, `anyhow` for CLI
- **Lints**: pedantic clippy via `[lints.clippy]` in `Cargo.toml`
- **Formatting**: nightly rustfmt, edition 2024, 120 char max, Unix newlines
- **Documentation language**: British English
- **Doc comments**: required on public items, prefer "why-first" style

## Git Workflow

Trunk-based: `main` is the sole long-lived branch. Feature work via short-lived branches + PRs.

- Commit format: `type(scope): description` (conventional commits, no emoji)
- release-plz auto-bumps version and creates tags from conventional commits

## CI

**Woodpecker CI** (primary — GitHub forge):

- **CI** (`ci.yaml`): fmt (nightly), clippy (stable), test-glibc, test-musl, docs, cargo-audit, cargo-deny
- **Release-plz** (`release-plz.yaml`): auto version bump + tag on main push
- **Release** (`release.yaml`): Linux x86_64/aarch64 glibc+musl builds, crates.io publish, GitHub Release

## Dev Environment

```bash
# Install tools
mise install

# Pin stable Rust + components
# (handled by rust-toolchain.toml)
rustup toolchain install nightly --component rustfmt  # for fmt
```

## Style

- Use British English in all prose (e.g. colour, organisation, behaviour).

# AGENTS — Repository Enforcement Contract

This file is the **authoritative policy** for all automation touching this repo
(CI, bots, and AI coding agents). Violations are defects.

## 1) Scope

Repository type: **Public Rust CLI and library crate (GPL-2.0-or-later)**.

- Binary: `cpuid2cpuflags` — Gentoo CPU_FLAGS_* generator
- Library: `cpuid2cpuflags` — reusable CPU flag detection and CFLAGS conversion
- Upstream reference: [gentoo/cpuid2cpuflags](https://github.com/gentoo/cpuid2cpuflags) (hard fork)

Core principles:

- Idiomatic, safe Rust
- Feature-gated optional dependencies
- Output compatibility with upstream C version
- Flag tables must match Gentoo profiles/desc definitions
- Conventional commits for automated releases

## 2) Hard Rules

### 2.1 No unsafe code (with documented exception)

`unsafe` is **denied** via `[lints.rust]` in `Cargo.toml`.

The sole permitted exception is the hwcap detection module which calls
`libc::getauxval()`. This module must have `#[allow(unsafe_code)]` with
full `// SAFETY:` documentation.

No other module may use `unsafe`. If a future change requires it:

- Add a `// SAFETY:` comment documenting invariants
- Open a tracking issue explaining why safe alternatives are insufficient
- Require explicit human approval before merging

### 2.2 Flag table accuracy

Flag definitions in `tables/*.rs` MUST match the corresponding
`profiles/desc/cpu_flags_*.desc` files in the Gentoo repository.
Test data from upstream MUST pass without modification.

Adding new flags that Gentoo has not yet accepted MUST be gated behind
a feature flag (e.g. `riscv`, `staging-flags`).

### 2.3 Output compatibility

For any CPU whose register dump matches an upstream test case, the output
MUST be byte-identical to upstream `cpuid2cpuflags`. The test suite enforces
this via the ported test data.

### 2.4 Feature-gated dependencies

New dependencies MUST be `optional = true` and gated behind a feature in
`Cargo.toml` unless they are core CPU detection (always compiled).

Do not add non-optional dependencies without explicit justification.

### 2.5 No root required

The binary must never require root, setuid, or elevated privileges.
All CPU detection methods (CPUID, getauxval, /proc/self/auxv, uname,
/proc/cpuinfo) are unprivileged by design.

### 2.6 Lints live in Cargo.toml

All clippy and rustc lint configuration lives in the `[lints]` section of
`Cargo.toml`. Do not use `.cargo/config.toml` for lint flags.

### 2.7 Conventional commits

Format: `type(scope): description`

Types: `feat`, `fix`, `refactor`, `docs`, `style`, `test`, `ci`, `chore`, `perf`

No emoji prefixes. release-plz parses these for automated version bumps.

### 2.8 British English

All prose — doc comments, commit messages, documentation — uses British English
(e.g. colour, behaviour, organisation).

### 2.9 Documentation

All public items must have doc comments. Prefer "why-first" style.

### 2.10 Test fixture integrity

Agents MUST NOT modify upstream test data files in `tests/fixtures/`.
These are the compatibility contract with the upstream C implementation.

## 3) CI Contract

| Check | Toolchain | Command | Blocking |
|-------|-----------|---------|----------|
| Format | nightly | `cargo fmt --all --check` | Yes |
| Clippy | stable | `cargo clippy --all-targets --all-features` | Yes |
| Test (glibc) | stable | `cargo test --all-features` | Yes |
| Test (musl) | stable | `cargo test --all-features` | Yes |
| Docs | stable | `cargo doc --no-deps --all-features` (`-D warnings`) | Yes |
| MSRV | 1.85.0 | `cargo check --all-features` | Yes |
| Audit | — | `cargo-audit` + `cargo-deny` | Yes |

## 4) Decision Authority

- `Cargo.toml` is the source of truth for dependencies, features, and lints
- `rust-toolchain.toml` pins the default toolchain channel
- `deny.toml` governs licence and advisory policy
- `release-plz.toml` controls release automation

## 5) Release Flow

1. Conventional commits land on `main`
2. `release-plz` analyses commits, bumps `Cargo.toml` version, creates `v*` tag
3. Tag push triggers the Release workflow: Linux builds + GitHub Release + crates.io publish
4. Agents must not create tags manually

## 6) Agent Constraints

Agents MUST:

- Verify flag table changes against Gentoo profiles/desc
- Run `cargo clippy` and `cargo test` before considering work done
- Propose changes as diffs
- Verify assumptions before implementation
- Avoid speculative refactors

Agents MUST NOT:

- Silence failing checks or suppress clippy lints without justification
- Add `unsafe` code outside the hwcap module
- Create version tags or publish to crates.io
- Add non-optional dependencies without feature gating
- Modify upstream test data files (they are the compatibility contract)

If an agent cannot comply, it must fail loudly and request human intervention.
Silent policy bypass is forbidden.

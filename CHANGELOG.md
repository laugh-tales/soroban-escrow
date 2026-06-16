# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Versioning strategy:** Versions are tagged as `vMAJOR.MINOR.PATCH`.
> - **PATCH** — backward-compatible bug fixes.
> - **MINOR** — new backward-compatible functionality.
> - **MAJOR** — breaking changes to the contract interface or storage layout.

---

## [Unreleased]

## [0.1.0] - 2026-06-17

### Added

#### Escrow Smart Contract (`contracts/escrow`)
- `initialize(admin)` — deploy-time setup; stores the admin address and initialises the escrow counter.
- `create_escrow(depositor, beneficiary, token, amount, release_time)` — locks SAC/SEP-41 tokens into the contract and emits an `escrow_created` event; returns the new `escrow_id`.
- `release(escrow_id)` — depositor-only; transfers funds to the beneficiary once `release_time` is reached; emits an `escrow_released` event.
- `dispute(escrow_id)` — beneficiary-only; transitions an Active escrow to Disputed status; emits an `escrow_disputed` event.
- `refund(escrow_id)` — admin-only; returns funds to the depositor for Active or Disputed escrows; emits an `escrow_refunded` event.
- `get_escrow(escrow_id)` — read-only query returning full escrow details (`Escrow` struct).
- `get_count()` — read-only query returning the total number of escrows created.
- `EscrowStatus` enum: `Active`, `Released`, `Refunded`, `Disputed`.
- On-chain persistent storage per escrow (`depositor`, `beneficiary`, `token`, `amount`, `release_time`, `status`).
- Amount validation — rejects escrows with `amount ≤ 0`.
- Time-lock enforcement — `release()` panics if called before `release_time`.

#### Rust Utility Library (`src/`)
- `address` — Stellar address validation helpers.
- `asset` — Asset/token formatting and parsing utilities.
- `encoding` — Base64 and hex encoding/decoding.
- `hash` — SHA-256 hashing wrappers.
- `memo` — Stellar memo type utilities.
- `transaction` — Transaction construction helpers.
- CLI entry point (`src/main.rs`) via `clap` for tool invocation from the command line.

#### Tests
- Unit tests for `create_escrow`, `release`, `dispute`, `refund` using the Soroban test environment.
- Negative-path tests: zero-amount rejection and release-time enforcement.

#### Project Infrastructure
- `Cargo.toml` workspace configuration for `soroban-toolkit` and `contracts/escrow`.
- GitHub Actions CI workflow (`.github/workflows/ci.yml`) running `cargo test` and `cargo clippy`.
- `README.md` with project overview, contract function reference, lifecycle diagram, build and deploy instructions.
- `ARCHITECTURE.md` describing on-chain state layout and file structure.
- `CONTRIBUTING.md` with setup and contribution guidelines.
- MIT `LICENSE`.

---

[Unreleased]: https://github.com/laugh-tales/soroban-toolkit/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/laugh-tales/soroban-toolkit/releases/tag/v0.1.0

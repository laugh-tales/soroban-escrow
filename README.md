# Soroban Escrow

A trustless escrow smart contract built on the Stellar blockchain using Soroban. Lock funds, set conditions, and release payments — no intermediaries needed.

[![CI](https://github.com/laugh-tales/soroban-escrow/actions/workflows/ci.yml/badge.svg)](https://github.com/laugh-tales/soroban-escrow/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## What It Does

Soroban Escrow enables trustless peer-to-peer transactions on Stellar. A depositor locks funds into the contract specifying a beneficiary, token, amount, and release time. Funds are held securely until conditions are met — no custodian, no trust required.

**Use cases:**
- Freelance payments — pay only when work is delivered
- P2P trading — exchange assets without trusting a counterparty
- Milestone-based contracts — release funds as milestones are completed
- Dispute resolution — built-in dispute mechanism with admin arbitration

## Contract Functions

| Function | Who Calls It | Description |
|---|---|---|
| `initialize(admin)` | Deployer | Set up the contract with an admin address |
| `create_escrow(depositor, beneficiary, token, amount, release_time)` | Depositor | Lock funds into escrow |
| `release(escrow_id)` | Depositor | Release funds to beneficiary after release time |
| `dispute(escrow_id)` | Beneficiary | Raise a dispute for admin review |
| `refund(escrow_id)` | Admin | Refund depositor on Active or Disputed escrow |
| `get_escrow(escrow_id)` | Anyone | Query escrow details and status |
| `get_count()` | Anyone | Get total number of escrows created |

## Escrow Lifecycle
Depositor                  Contract                Beneficiary
|                          |                        |
|--- create_escrow() ----> |                        |
|                          | (funds locked)         |
|                          |                        |
|--- release() ----------> |                        |
|                          |--- transfer funds ---> |
|                          |                        |
OR if disputed:
|                          | <-- dispute() ---------|
|                          | (status: Disputed)     |
|                          |                        |
Admin                       |                        |
|--- refund() -----------> |                        |
|                          |--- return funds -----> Depositor

## Build

**Prerequisites:**
- Rust 1.75+
- Soroban CLI ([install guide](https://developers.stellar.org/docs/smart-contracts/getting-started/setup))

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Clone the repo
git clone https://github.com/laugh-tales/soroban-toolkit
cd soroban-toolkit

# Run tests
cargo test

# Build the contract
soroban contract build --manifest-path contracts/escrow/Cargo.toml
```

## Deploy to Testnet

```bash
# Configure testnet identity (first time only)
soroban keys generate --global myaccount --network testnet

# Deploy
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroban_escrow.wasm \
  --network testnet \
  --source myaccount
```

## Project Structure
soroban-toolkit/
├── contracts/
│   └── escrow/              # Soroban escrow smart contract (Rust)
│       ├── src/
│       │   └── lib.rs       # Contract implementation + tests
│       └── Cargo.toml
├── src/                     # Rust utility library for Soroban developers
├── .github/
│   └── workflows/
│       └── ci.yml           # CI — runs cargo test and cargo clippy
├── Cargo.toml
└── README.md

## Contributing

This project participates in the [Stellar Wave Program](https://drips.network/wave/stellar) on Drips. Contributors can earn USDC rewards for resolving issues.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for setup and contribution guidelines.

Browse [open issues](https://github.com/laugh-tales/soroban-toolkit/issues) — labeled by complexity:
- 🟢 `good first issue` — Small fixes, docs, tests (100 pts)
- 🟡 `enhancement` — New features, improvements (150 pts)
- 🔴 `high complexity` — Complex features (200 pts)

## License

MIT — see [LICENSE](./LICENSE) for details.

## Links

- [Stellar Developers](https://developers.stellar.org)
- [Soroban Documentation](https://soroban.stellar.org)
- [Drips Wave Program](https://drips.network/wave/stellar)

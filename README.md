# StellarProof — On-Chain Digital Content Provenance

StellarProof mints **immutable provenance certificates** for digital media on **Stellar Soroban**. Creators hash content locally, attach metadata manifests, and anchor authenticity on-chain — addressing deepfakes and AI-generated misinformation.

## What's built today (Phase 1)

| Component | Status |
|-----------|--------|
| **Frontend** | Next.js landing page, manifest generator, creator upload wizard, Freighter wallet |
| **Contracts** | `stellarproof` (hash verify + mint), `provenance` (certificates), `registry` (TEE attestation) |
| **Deploy** | `scripts/deploy-testnet.sh` for testnet |
| **CI** | Contract tests + frontend lint/test/build |

## Quick start

### Prerequisites

- [Node.js](https://nodejs.org/) 20+
- [pnpm](https://pnpm.io/) 10+
- [Rust](https://rustup.rs) 1.84+ with `wasm32v1-none` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli) (optional, for deploy)

### Setup

```bash
git clone https://github.com/tommydebisi/StellarProof.git
cd StellarProof
pnpm install
rustup target add wasm32v1-none
```

### Run frontend

```bash
pnpm --filter web dev
```

Open http://localhost:3000 — try `/creator/upload-content` and `/manifest`.

### Run tests

```bash
pnpm test:contracts   # Soroban contract unit tests
pnpm test:frontend    # Jest (SHA-256 validation)
pnpm build:frontend   # Next.js production build
```

### Deploy contracts (testnet)

```bash
./scripts/deploy-testnet.sh
```

Writes contract IDs to `.stellarproof/deployments.json`.

## Contracts

| Contract | Purpose |
|----------|---------|
| `stellarproof` | SHA-256 content verification; calls provenance to mint on match |
| `provenance` | Stores provenance certificates (hash, metadata, owner, timestamp) |
| `registry` | TEE provider registry and attestation verification |
| `oracle` | Placeholder (Phase 2) |

Build all WASM:

```bash
make -C contracts build
```

## Contributing & Stellar Wave

We participate in the [Stellar Wave Program](https://www.drips.network/wave/stellar). See [CONTRIBUTING.md](CONTRIBUTING.md) and issues tagged `wave-candidate`.

Browse the planned backlog in [docs/WAVE_ISSUES.md](docs/WAVE_ISSUES.md).

## Roadmap

Phase 2+ (oracle worker, verification APIs, IPFS, TEE integration) is documented in [docs/ROADMAP.md](docs/ROADMAP.md).

## License

MIT — see [LICENSE](LICENSE).

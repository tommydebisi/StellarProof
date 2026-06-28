# StellarProof Roadmap

## Phase 1 — Core provenance loop (current)

- [x] Soroban contracts: verification, certificates, TEE registry
- [x] Frontend: landing, manifest generator, upload wizard
- [x] Local SHA-256 hashing (64-char hex, aligned with contracts)
- [x] Cross-contract mint: stellarproof → provenance
- [x] Testnet deploy script
- [ ] Freighter-signed on-chain mint from frontend
- [ ] Contract events decoder in frontend

## Phase 2 — Verification pipeline

- [ ] Oracle worker (hash manifest, submit attestation to registry)
- [ ] `/api/verify/submit` and `/api/verify/status/:jobId`
- [ ] Registry → provenance mint on `Verified` attestation
- [ ] IPFS adapter for manifest/media metadata

## Phase 3 — Enterprise & trust

- [ ] AWS Nitro TEE attestation integration
- [ ] KMS encryption layer for media
- [ ] Proof-as-a-Service webhooks
- [ ] MongoDB storage option for high-throughput queries

## Phase 4 — Ecosystem

- [ ] SDK for third-party integrators
- [ ] Certificate verification API
- [ ] Multi-network deploy (testnet + mainnet CI)

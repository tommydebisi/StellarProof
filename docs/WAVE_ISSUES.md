# Stellar Wave Issue Backlog

Copy these into GitHub Issues before reapplying to the [Stellar Wave Program](https://www.drips.network/wave/stellar). Label each with `wave-candidate` and the appropriate `complexity:*` label.

## Trivial (100 pts)

1. **Add GitHub labels** — Create `complexity:trivial`, `complexity:medium`, `complexity:high`, `wave-candidate`, `bug`.
2. **Fix Header `/register` link** — Add stub page or remove broken nav link.
3. **Document manifest JSON schema** — Add `docs/MANIFEST_SCHEMA.md` with example payload.
4. **Add `.env.example`** — Document optional frontend env vars.

## Medium (150 pts)

5. **Freighter wallet in upload wizard** — Show connected address and include in manifest export.
6. **Manifest export/download** — Download generated manifest as JSON from modal.
7. **Contract events: CertificateMinted** — Emit structured event from provenance `mint()`.
8. **Replace oracle placeholder** — Stub contract that accepts verification requests.
9. **Root workspace profile cleanup** — Move `[profile.release]` to workspace root Cargo.toml.
10. **Frontend invoke helper** — TS module to call `verify_and_mint` via RPC using `.stellarproof/deployments.json`.
11. **Upload wizard step indicator** — Progress UI for the 5 wizard sections.
12. **Registry admin docs** — Document `add_tee_hash` and provider setup in `contracts/registry/README.md`.

## High (200 pts)

13. **End-to-end testnet invoke script** — Node script: deploy → verify content → assert certificate ID.
14. **Registry → provenance on Verified** — Cross-contract mint when attestation passes all checks.
15. **`/api/verify/submit` MVP** — In-memory job queue with status endpoint.
16. **Oracle worker MVP** — New `oracle-worker/` package: fetch manifest, hash, submit to registry.

## Application text (Planned issues field)

> StellarProof provides on-chain digital content provenance for the Stellar ecosystem. Contributors work across Soroban contracts (verification, certificate minting, TEE registry), Next.js frontend (creator upload wizard, manifest generator, Freighter wallet), and testnet integration. Phase 1 delivers the core loop: hash content → verify on-chain → mint provenance certificate. Issues are scoped for single Wave sprints with tests and clear acceptance criteria.

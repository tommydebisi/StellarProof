# Contributing to StellarProof

Thank you for contributing to **StellarProof** — on-chain digital content provenance for the Stellar ecosystem.

## Getting started

1. Fork and clone: `git clone https://github.com/tommydebisi/StellarProof.git`
2. Install: `pnpm install`
3. Add WASM target: `rustup target add wasm32v1-none`
4. Branch: `git checkout -b feature/my-change`

## Development

| Path | Purpose |
|------|---------|
| `frontend/` | Next.js app (landing, manifest, upload wizard) |
| `contracts/` | Soroban Rust workspace (provenance, stellarproof, registry, oracle) |
| `scripts/` | Testnet deploy |
| `docs/` | Roadmap and Wave issue backlog |

### Run locally

```bash
pnpm --filter web dev          # http://localhost:3000
pnpm test:contracts            # cargo test --workspace
pnpm test:frontend             # Jest
pnpm build:frontend
make -C contracts build        # WASM artifacts
```

### Deploy (testnet)

```bash
./scripts/deploy-testnet.sh
```

## Pull request guidelines

- **One concern per PR** — easier to review during Wave sprints
- Contract changes require `cargo test --workspace`
- Frontend changes require `pnpm test:frontend` and `pnpm check:frontend`
- No secrets in commits (`.stellarproof/deployments.json` is gitignored)

## Stellar Wave Program

We participate in [Drips Wave](https://www.drips.network/wave/stellar). Issues tagged `wave-candidate` are eligible for contributor rewards during active Waves.

| Label | Points | Scope |
|-------|--------|-------|
| `complexity:trivial` | 100 | Docs, typos, small fixes |
| `complexity:medium` | 150 | Features, contract changes, frontend work |
| `complexity:high` | 200 | Multi-package integrations, APIs, oracle worker |

Browse [docs/WAVE_ISSUES.md](docs/WAVE_ISSUES.md) for the planned backlog. Use the **Wave candidate** issue template when proposing new work.

During an active Wave, comment on an issue to apply. Maintainers assign contributors within 24–48 hours.

## Questions

Open a [GitHub Discussion](https://github.com/tommydebisi/StellarProof/discussions) or ask in the Stellar Developer Discord.

## License

Contributions are licensed under the MIT License.

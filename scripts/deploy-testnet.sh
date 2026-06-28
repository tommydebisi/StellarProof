#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

NETWORK="${NETWORK:-testnet}"
SOURCE="${SOURCE:-stellarproof}"
TARGET="wasm32v1-none"

if ! command -v stellar &>/dev/null; then
  echo "Stellar CLI required: https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli"
  exit 1
fi

if ! stellar keys address "$SOURCE" &>/dev/null; then
  stellar keys generate "$SOURCE"
fi

make -C contracts build

mkdir -p .stellarproof
DEPLOY_FILE=".stellarproof/deployments.json"
WASM_DIR="target/${TARGET}/release"

provenance_id=$(stellar contract deploy \
  --wasm "${WASM_DIR}/provenance.wasm" \
  --source-account "$SOURCE" \
  --network "$NETWORK" \
  --alias stellarproof_provenance 2>&1 | tail -1)

stellarproof_id=$(stellar contract deploy \
  --wasm "${WASM_DIR}/stellarproof.wasm" \
  --source-account "$SOURCE" \
  --network "$NETWORK" \
  --alias stellarproof_verify 2>&1 | tail -1)

stellar contract invoke \
  --id "$provenance_id" \
  --source-account "$SOURCE" \
  --network "$NETWORK" \
  -- initialize --oracle "$stellarproof_id"

stellar contract invoke \
  --id "$stellarproof_id" \
  --source-account "$SOURCE" \
  --network "$NETWORK" \
  -- initialize --provenance_address "$provenance_id"

cat > "$DEPLOY_FILE" <<EOF
{
  "network": "$NETWORK",
  "contracts": {
    "provenance": "$provenance_id",
    "stellarproof": "$stellarproof_id"
  }
}
EOF

echo "Deployed to $NETWORK:"
cat "$DEPLOY_FILE"

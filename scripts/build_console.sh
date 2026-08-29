#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FIRST_SYNTH_DIR="${REPO_ROOT}/../first-synth/console"

echo "==> 1. Building monist-wasm package (--target web)..."
wasm-pack build --target web "${REPO_ROOT}/crates/monist-wasm"

echo "==> 2. Bundling monist-console React app..."
(cd "${REPO_ROOT}/tools/monist-console" && npm run build)

if [ -d "${FIRST_SYNTH_DIR}" ]; then
  echo "==> 3. Syncing build artifacts to first-synth/console/..."
  cp -r "${REPO_ROOT}/tools/monist-console/dist/"* "${FIRST_SYNTH_DIR}/"
  echo "==> Done! Console build synced to first-synth/console/."
else
  echo "==> first-synth/console not found at ${FIRST_SYNTH_DIR}, skipped sync."
fi

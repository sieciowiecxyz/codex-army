#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

exec "$repo_root/scripts/army-cargo.sh" build \
  --locked \
  --profile release-army \
  -p codex-cli \
  --bin codex \
  -p codex-code-mode-host \
  --bin codex-code-mode-host \
  "$@"

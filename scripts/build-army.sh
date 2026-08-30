#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

exec cargo build \
  --locked \
  --manifest-path "$repo_root/codex-source/codex-rs/Cargo.toml" \
  --profile release-army \
  -p codex-cli \
  --bin codex \
  -p codex-code-mode-host \
  --bin codex-code-mode-host \
  "$@"

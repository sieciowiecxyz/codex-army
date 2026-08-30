#!/usr/bin/env bash
set -euo pipefail

codex_bin=${1:?usage: health-check-army.sh CODEX HOST}
host_bin=${2:?usage: health-check-army.sh CODEX HOST}

run_clean() {
  local label=$1
  shift
  local output
  output="$("$@" 2>&1)"
  if grep -Eiq '(^|[^[:alpha:]])warning([^[:alpha:]]|$)' <<<"$output"; then
    printf '%s emitted a warning:\n%s\n' "$label" "$output" >&2
    exit 1
  fi
  test -n "$output"
}

test -x "$codex_bin"
test -x "$host_bin"
run_clean codex "$codex_bin" --version
run_clean codex "$codex_bin" --help
run_clean codex-code-mode-host "$host_bin" --help

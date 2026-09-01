#!/usr/bin/env bash
set -euo pipefail

codex_bin=${1:?usage: test-account-selection.sh CODEX}
test -x "$codex_bin"

test_root=$(mktemp -d "${TMPDIR:-/tmp}/codex-army-account-selection.XXXXXX")
trap 'rm -rf -- "$test_root"' EXIT
mkdir -p "$test_root/bin"
marker="$test_root/invoked"

cat >"$test_root/bin/codex-accounts" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
[[ "${1:-}" == "use-best" && "$#" == 1 ]]
touch "$CODEX_ARMY_ACCOUNT_SELECTION_MARKER"
EOF
chmod 755 "$test_root/bin/codex-accounts"

PATH="$test_root/bin:$PATH" \
  CODEX_ARMY_ACCOUNT_SELECTION_MARKER="$marker" \
  "$codex_bin" completion bash >/dev/null

test -f "$marker"

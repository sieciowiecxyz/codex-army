#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
upstream_root="$repo_root/codex-source/codex-rs"
patch_root="$repo_root/patches/army"
target_dir="${CARGO_TARGET_DIR:-$upstream_root/target}"

[[ -f "$upstream_root/Cargo.toml" ]] || {
  echo "upstream Codex source is missing: $upstream_root" >&2
  exit 1
}

prepared_root="$(mktemp -d "${TMPDIR:-/tmp}/codex-army-source.XXXXXX")"
cleanup() {
  rm -rf -- "$prepared_root"
}
trap cleanup EXIT

# Keep codex-source pristine. The temporary tree is the only tree Cargo sees.
tar -C "$upstream_root" --exclude=target -cf - . | tar -C "$prepared_root" -xf -

shopt -s nullglob
army_patches=("$patch_root"/*.patch)
shopt -u nullglob
if ((${#army_patches[@]} == 0)); then
  echo "no Army patches found in $patch_root" >&2
  exit 1
fi

for patch in "${army_patches[@]}"; do
  git -C "$prepared_root" apply --whitespace=error -p1 "$patch"
done

mkdir -p "$target_dir"
export CARGO_TARGET_DIR="$target_dir"

args=("$@")
[[ -n "${args[0]:-}" ]] || {
  echo "usage: army-cargo.sh <cargo-subcommand> [args...]" >&2
  exit 2
}

if [[ "${args[0]}" == "install" ]]; then
  for ((index = 0; index < ${#args[@]}; index++)); do
    if [[ "${args[index]}" == "--path" ]] && ((index + 1 < ${#args[@]})); then
      path="${args[index + 1]}"
      if [[ "$path" != /* ]]; then
        args[index + 1]="$prepared_root/$path"
      fi
    fi
  done
  exec cargo "${args[@]}"
fi

exec cargo "${args[0]}" --manifest-path "$prepared_root/Cargo.toml" "${args[@]:1}"

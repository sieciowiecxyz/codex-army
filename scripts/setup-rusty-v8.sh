#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
target="${RUSTY_V8_TARGET:-$(rustc -vV | sed -n 's/^host: //p')}"

version="$(python3 "${repo_root}/codex-source/.github/scripts/rusty_v8_bazel.py" resolved-v8-crate-version)"
release_tag="rusty-v8-v${version}"
base_url="https://github.com/openai/codex/releases/download/${release_tag}"

cache_dir="${XDG_CACHE_HOME:-${HOME}/.cache}/codex-army/rusty-v8"
mkdir -p "${cache_dir}"

profile="ptrcomp_sandbox_release"
archive_name="librusty_v8_${profile}_${target}.a.gz"
binding_name="src_binding_${profile}_${target}.rs"
checksums_name="rusty_v8_${profile}_${target}.sha256"

archive_path="${cache_dir}/${archive_name}"
binding_path="${cache_dir}/${binding_name}"
checksums_path="${cache_dir}/${checksums_name}"

if [[ ! -f "${archive_path}" || ! -f "${binding_path}" || ! -f "${checksums_path}" ]]; then
  curl -fsSL "${base_url}/${archive_name}" -o "${archive_path}.tmp"
  curl -fsSL "${base_url}/${binding_name}" -o "${binding_path}.tmp"
  curl -fsSL "${base_url}/${checksums_name}" -o "${checksums_path}.tmp"
  mv "${archive_path}.tmp" "${archive_path}"
  mv "${binding_path}.tmp" "${binding_path}"
  mv "${checksums_path}.tmp" "${checksums_path}"
fi

if [[ "$(wc -l < "${checksums_path}")" -ne 2 ]]; then
  echo "Expected exactly two checksums for ${target} in ${checksums_path}" >&2
  exit 1
fi

# Existing Windows-built release manifests use CRLF line endings.
if command -v sha256sum >/dev/null 2>&1; then
  (cd "${cache_dir}" && tr -d '\r' < "${checksums_path}" | sha256sum -c -) >/dev/null
else
  (cd "${cache_dir}" && tr -d '\r' < "${checksums_path}" | shasum -a 256 -c -) >/dev/null
fi

case "${1:-}" in
  --archive) printf '%s\n' "${archive_path}" ;;
  --binding) printf '%s\n' "${binding_path}" ;;
  *) printf 'RUSTY_V8_ARCHIVE=%s RUSTY_V8_SRC_BINDING_PATH=%s\n' \
      "${archive_path}" "${binding_path}" ;;
esac

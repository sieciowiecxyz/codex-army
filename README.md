# Codex Army

Codex Army is a maintained fork of Codex with account-switch failover. The
upstream Codex checkout is kept in [`codex-source/`](codex-source/); Army-owned
automation and packaging stay at the repository root.

## Layout

- `codex-source/` — upstream Codex source snapshot and its original tooling;
- `patches/army/` — ordered patches maintained by Army and applied only to a
  temporary build tree;
- `packaging/` — Fedora RPM and Arch package definitions;
- `.github/workflows/ci.yml` — the single Army build, test, package, and release workflow;
- `justfile` — stable root commands that do not depend on the upstream directory layout.

## Build locally

```sh
just build-army
just install-army
```

The optimized `release-army` profile uses fat LTO, one codegen unit, symbol
stripping, and disabled incremental compilation. The release workflow runs on
tags named `army-v*`, builds the binary, creates Fedora and Arch packages, and
publishes all artifacts to the GitHub Release for that tag.

All root build and test commands apply `patches/army/` to a temporary copy of
`codex-source/codex-rs`; the tracked upstream snapshot is never modified by
the build.

Code-mode waits use a runtime-owned schedule of `30s → 1m → 2m → 4m → 8m →
10m`, then check the still-running task every 10 minutes. The model's
`yield_time_ms` does not control this polling loop; task output is buffered
with a hard limit and returned at completion or the next 10-minute checkpoint.

Account failover uses the companion `codex-accounts` command. Auto-prompt
functionality is intentionally not part of this fork.

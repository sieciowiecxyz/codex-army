# Codex Army

Codex Army is a small maintained fork of Codex with account-switch failover.
Auto-prompt is intentionally not included. The upstream Codex checkout is
kept in [`codex-source/`](codex-source/); Army-owned changes stay at the
repository root.

## Layout

- `codex-source/` — pristine upstream Codex source snapshot;
- `patches/army/` — ordered Army patches applied to a temporary build tree;
- `scripts/` — patched Cargo wrapper, V8 setup, build, and health-check scripts;
- `packaging/` — Fedora RPM and Arch package definitions;
- `.github/workflows/ci.yml` — formatting, smoke build, optimized release, and packaging workflow;
- `justfile` — stable root commands for local development and releases.

## Build locally

```sh
just build-army              # fast, unoptimized local build
just build-army-optimized    # optimized release-army build
just install-army             # install optimized Army binaries
```

The optimized `release-army` profile uses fat LTO, one codegen unit, symbol
stripping, and disabled incremental compilation. The release workflow runs on
tags named `army-v*`, builds both `codex` and `codex-code-mode-host`, creates
Fedora and Arch packages, and publishes all artifacts to the GitHub Release.

All root build and test commands apply `patches/army/` to a temporary copy of
`codex-source/codex-rs`; the tracked upstream snapshot is never modified by
the build. Upstream updates require refreshing the snapshot and rebasing the
Army patch, rather than editing `codex-source/` directly.

## Army patches

The main patch adds account-switch failover and mock coverage, recovers
incomplete tool-call history with an aborted output, and changes Code Mode
waiting to a runtime-owned `30s → 1m → 2m → 4m → 8m → 10m` schedule. The
model's `yield_time_ms` is ignored; output resets the schedule to 30 seconds,
and buffered output has a hard limit.

Account failover uses the companion `codex-accounts` command. Auto-prompt
functionality is intentionally not part of this fork. CI currently runs
formatting and a Codex smoke build; it does not execute the test suite. Tests
remain available locally through `just test`.

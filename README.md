# Codex Army

Codex Army is a maintained fork of Codex with account-switch failover. The
upstream Codex checkout is kept in [`codex-source/`](codex-source/); Army-owned
automation and packaging stay at the repository root.

## Layout

- `codex-source/` — upstream Codex source snapshot and its original tooling;
- `patches/` — patches maintained by Army;
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

Account failover uses the companion `codex-accounts` command. Auto-prompt
functionality is intentionally not part of this fork.

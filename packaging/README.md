# Codex Army packages

The release workflow builds the Linux binary with the `release-army` Cargo
profile and packages the same bytes as an RPM and an Arch Linux package.

Both packages install the fork as `/usr/bin/codex` and conflict with a package
that owns the official `codex` executable. The source package recipes are
deliberately binary-only: compilation happens once in CI, while packaging is
just staging and metadata generation.

Account failover invokes the companion `codex-accounts use-best` command. Keep
that companion installed alongside the fork; without it, normal Codex usage
still works but automatic account failover is unavailable.

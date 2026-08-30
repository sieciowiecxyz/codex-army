# Codex Army packages

The release workflow builds the Linux binaries with the `release-army` Cargo
profile and packages the same bytes as an RPM and an Arch Linux package.

Both packages install the fork as `/usr/bin/codex` together with the Code Mode
host as `/usr/bin/codex-code-mode-host`, and conflict with a package that owns
the official `codex` executable. The source package recipes are deliberately
binary-only: compilation happens once in CI, while packaging is just staging
and metadata generation.

Code Mode resolves `codex-code-mode-host` next to the running `codex`
executable, so both binaries must always be shipped together. Keep them
installed alongside each other; without the host, Code Mode fails with `host
executable was not found`.

Account failover invokes the companion `codex-accounts use-best` command. Keep
that companion installed alongside the fork; without it, normal Codex usage
still works but automatic account failover is unavailable.

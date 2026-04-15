# Codex Army Fork

This fork keeps a minimal diff against upstream `openai/codex` and currently adds two behaviors:

- automatic account failover on usage/rate-limit via `codex-accounts`
- `/autoprompt` session mode that keeps nudging the agent until it returns `done`, `blocked`, or fails JSON parsing repeatedly

## Install

```bash
cd codex-rs
cargo install --path cli --bin codex-army --force --locked
```

## Goals

- keep the fork mergeable with upstream
- keep changes local to TUI slash-command flow and core retry/auth flow
- avoid broad repo-wide rebranding

## Fork surface

- `codex-rs/cli`: lightweight `codex-army` binary target and CLI naming
- `codex-rs/tui`: `/autoprompt` command and session loop
- `codex-rs/core` plus `codex-rs/codex-api`: account failover on usage/rate limits

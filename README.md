# Codex Army

Codex Army is a lightweight fork of OpenAI Codex focused on one practical goal:
make the agent more persistent and more useful for long implementation tasks.

This fork intentionally keeps a small diff against upstream so it is still
reasonable to merge newer upstream versions from time to time.

## What is different from upstream

Codex Army currently adds three fork-specific features:

### 1. Automatic account failover with `codex-accounts`

Codex Army integrates with
[`codex-accounts`](https://github.com/sieciowiecxyz/codex-accounts)
to automatically switch ChatGPT accounts when the current account hits a usage
limit or rate limit.

Why this exists:

- upstream Codex stops working when the active account runs out of usable quota
- many people already rotate between multiple ChatGPT accounts
- switching accounts manually breaks flow and wastes time

What Codex Army does:

- detects usage-limit / rate-limit failures
- calls `codex-accounts` to switch to another available account
- reloads auth
- retries the same turn so the session can continue instead of stopping

### 2. `/autoprompt` mode

Codex Army adds an `/autoprompt` mode that keeps nudging the model until it
either:

- finishes the task
- reports that it is blocked
- or hits a safety cutoff for too many rapid auto-turns

Why this exists:

- upstream agents often stop too early
- the model may produce a partial implementation and act as if the work is done
- for long coding tasks you often want a stronger "keep going until it is really
  done" loop

What `/autoprompt` does:

- you give the agent a completion-check prompt
- if the task is truly finished, the model returns only `DONE`
- if the model is truly blocked, it returns only `BLOCKED`
- if the task is not finished and the model can still proceed, it does not need
  to return a status at all and should simply keep working
- Codex Army automatically re-prompts the same thread so the agent keeps going
  instead of stopping on a partial result
- if `/autoprompt` triggers three rapid auto-turns within one minute, it stops
  itself as a failsafe to avoid request spam

This is meant to reduce the common problem where the agent leaves work half-done
and exits too early.

### 3. Custom local models in `/model`

Codex Army can show user-defined local OSS models in the TUI `/model` picker.
Add `[[custom_models]]` entries to `~/.codex/config.toml` and point them at
LM Studio or Ollama. See [docs/custom-models.md](docs/custom-models.md).

## Install

Build and install the forked CLI binary as `codex`:

```bash
cd codex-rs
cargo install --path cli --bin codex --force --locked
```

Then run:

```bash
codex
```

## Relationship to upstream

This project is still based on OpenAI Codex:

- upstream repo: <https://github.com/openai/codex>
- upstream docs: <https://developers.openai.com/codex>

The fork tries to stay conservative:

- no broad repo-wide rebrand
- minimal CLI branding for `codex`
- fork logic kept mostly in TUI slash-command flow and auth retry flow

That does not guarantee conflict-free merges, but it is designed to make future
upstream updates easier than a large invasive fork.

## Extra notes

- companion project for account switching:
  [`codex-accounts`](https://github.com/sieciowiecxyz/codex-accounts)
- Rust workspace notes:
  [`codex-rs/README.md`](./codex-rs/README.md)

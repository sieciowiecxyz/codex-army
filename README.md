# Codex Army

Codex Army is a lightweight fork of OpenAI Codex focused on one practical goal:
make the agent more persistent and more useful for long implementation tasks.

This fork intentionally keeps a small diff against upstream so it is still
reasonable to merge newer upstream versions from time to time.

## What is different from upstream

Codex Army currently adds two fork-specific features:

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
- or fails the required JSON status format too many times

Why this exists:

- upstream agents often stop too early
- the model may produce a partial implementation and act as if the work is done
- for long coding tasks you often want a stronger "keep going until it is really
  done" loop

What `/autoprompt` does:

- you give the agent a completion-check prompt
- after each turn the model must return a small JSON status
- if the model says it should continue, Codex Army automatically prompts it to
  continue working
- the loop stops only when the model returns `done`, `blocked`, or repeatedly
  fails the JSON contract

This is meant to reduce the common problem where the agent leaves work half-done
and exits too early.

## Install

Build and install the forked CLI binary as `codex-army`:

```bash
cd codex-rs
cargo install --path cli --bin codex-army --force --locked
```

Then run:

```bash
codex-army
```

## Relationship to upstream

This project is still based on OpenAI Codex:

- upstream repo: <https://github.com/openai/codex>
- upstream docs: <https://developers.openai.com/codex>

The fork tries to stay conservative:

- no broad repo-wide rebrand
- minimal CLI branding for `codex-army`
- fork logic kept mostly in TUI slash-command flow and auth retry flow

That does not guarantee conflict-free merges, but it is designed to make future
upstream updates easier than a large invasive fork.

## Extra notes

- companion project for account switching:
  [`codex-accounts`](https://github.com/sieciowiecxyz/codex-accounts)
- Rust workspace notes:
  [`codex-rs/README.md`](./codex-rs/README.md)

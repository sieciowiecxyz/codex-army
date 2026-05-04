# Custom Local Models

Codex Army can pin local OSS models in the TUI `/model` picker with
`[[custom_models]]` entries in `~/.codex/config.toml`.

This does not download model weights into a Codex-owned `/models` directory.
Run the model in an OpenAI-compatible local provider first, then point Codex at
that provider.

## LM Studio

Start the LM Studio local server and use its OpenAI-compatible `/v1` endpoint:

```toml
[[custom_models]]
name = "Gemma Local"
model = "google/gemma-4-26b-a4b"
provider = "lmstudio"
base_url = "http://localhost:1234/v1"
description = "Local Gemma through LM Studio"
```

Then open Codex, run `/model`, and choose `Local / OSS: Gemma Local`.

When selected, Codex saves both:

```toml
model = "google/gemma-4-26b-a4b"
model_provider = "lmstudio"
```

When `base_url` matches the built-in provider endpoint, Codex reuses that stable
provider id. If `base_url` points somewhere else, Codex creates a generated
provider id such as `custom-model-0-lmstudio` for that entry. Requests go to the
selected provider URL, so LM Studio must stay running while Codex uses the model.

## Ollama

Use the built-in Ollama OSS provider when the default endpoint is enough:

```toml
[[custom_models]]
name = "Qwen Local"
model = "qwen2.5-coder:32b"
provider = "ollama"
description = "Local Qwen through Ollama"
```

Use `base_url` only when your provider runs somewhere other than the built-in
provider endpoint.

## Fields

- `name`: label shown in `/model`; defaults to `model`
- `model`: provider model id sent in API requests
- `provider`: `lmstudio` or `ollama`
- `base_url`: optional OpenAI-compatible endpoint override
- `description`: optional picker help text

Custom entries are appended under `Local / OSS` in `/model`. Built-in premium
models remain available, so switching back is just another `/model` selection.

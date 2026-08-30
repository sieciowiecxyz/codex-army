set positional-arguments := true

source_root := justfile_directory() / "codex-source"
rust_root := source_root / "codex-rs"
rusty_v8_script := justfile_directory() / "scripts" / "setup-rusty-v8.sh"
army_cargo := justfile_directory() / "scripts" / "army-cargo.sh"
rust_min_stack := "8388608"

# Army owns this file. The upstream justfile lives in codex-source/.

help:
    just -l

codex *args:
    {{ army_cargo }} run --locked --bin codex -- {{args}}

exec *args:
    {{ army_cargo }} run --locked --bin codex -- exec {{args}}

test *args:
    RUST_MIN_STACK={{ rust_min_stack }} NEXTEST_PROFILE=local {{ army_cargo }} nextest run --no-fail-fast {{args}}

fmt:
    {{ army_cargo }} fmt --all -- --config imports_granularity=Item

fmt-check:
    {{ army_cargo }} fmt --all -- --config imports_granularity=Item --check

fix *args:
    {{ army_cargo }} clippy --fix --tests --allow-dirty {{args}}

clippy *args:
    {{ army_cargo }} clippy --tests {{args}}

install:
    rustup show active-toolchain
    {{ army_cargo }} fetch --locked

# Fast local iteration build. GitHub Actions uses release-army explicitly below.
build-army:
    RUSTY_V8_ARCHIVE=`{{ rusty_v8_script }} --archive` RUSTY_V8_SRC_BINDING_PATH=`{{ rusty_v8_script }} --binding` {{ army_cargo }} build --locked -p codex-cli --bin codex -p codex-code-mode-host --bin codex-code-mode-host

build-army-optimized:
    RUSTY_V8_ARCHIVE=`{{ rusty_v8_script }} --archive` RUSTY_V8_SRC_BINDING_PATH=`{{ rusty_v8_script }} --binding` {{ army_cargo }} build --locked --profile release-army -p codex-cli --bin codex -p codex-code-mode-host --bin codex-code-mode-host

install-army:
    RUSTY_V8_ARCHIVE=`{{ rusty_v8_script }} --archive` RUSTY_V8_SRC_BINDING_PATH=`{{ rusty_v8_script }} --binding` {{ army_cargo }} install --path cli --locked --force --profile release-army --bin codex
    RUSTY_V8_ARCHIVE=`{{ rusty_v8_script }} --archive` RUSTY_V8_SRC_BINDING_PATH=`{{ rusty_v8_script }} --binding` {{ army_cargo }} install --path code-mode-host --locked --force --profile release-army --bin codex-code-mode-host

build-army-debug: build-army

write-config-schema:
    {{ army_cargo }} run --locked -p codex-core --bin codex-write-config-schema

write-app-server-schema *args:
    {{ army_cargo }} run --locked -p codex-app-server-protocol --bin write_schema_fixtures -- {{args}}

write-hooks-schema:
    {{ army_cargo }} run --locked -p codex-hooks --bin write_hooks_schema_fixtures

log *args:
    {{ army_cargo }} run --locked -p codex-cli --bin logs_client -- {{args}}

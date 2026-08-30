set positional-arguments := true

source_root := justfile_directory() / "codex-source"
rust_root := source_root / "codex-rs"
rusty_v8_script := justfile_directory() / "scripts" / "setup-rusty-v8.sh"
rust_min_stack := "8388608"

# Army owns this file. The upstream justfile lives in codex-source/.

help:
    just -l

codex *args:
    cargo run --locked --manifest-path {{ rust_root }}/Cargo.toml --bin codex -- {{args}}

exec *args:
    cargo run --locked --manifest-path {{ rust_root }}/Cargo.toml --bin codex -- exec {{args}}

test *args:
    RUST_MIN_STACK={{ rust_min_stack }} NEXTEST_PROFILE=local cargo nextest run --manifest-path {{ rust_root }}/Cargo.toml --no-fail-fast {{args}}

fmt:
    cargo fmt --manifest-path {{ rust_root }}/Cargo.toml --all -- --config imports_granularity=Item

fmt-check:
    cargo fmt --manifest-path {{ rust_root }}/Cargo.toml --all -- --config imports_granularity=Item --check

fix *args:
    cargo clippy --manifest-path {{ rust_root }}/Cargo.toml --fix --tests --allow-dirty {{args}}

clippy *args:
    cargo clippy --manifest-path {{ rust_root }}/Cargo.toml --tests {args}

install:
    rustup show active-toolchain
    cargo fetch --locked --manifest-path {{ rust_root }}/Cargo.toml

build-army:
    RUSTY_V8_ARCHIVE=`{{ rusty_v8_script }} --archive` RUSTY_V8_SRC_BINDING_PATH=`{{ rusty_v8_script }} --binding` cargo build --locked --manifest-path {{ rust_root }}/Cargo.toml --profile release-army -p codex-cli --bin codex -p codex-code-mode-host --bin codex-code-mode-host

install-army:
    RUSTY_V8_ARCHIVE=`{{ rusty_v8_script }} --archive` RUSTY_V8_SRC_BINDING_PATH=`{{ rusty_v8_script }} --binding` cargo install --path {{ rust_root }}/cli --locked --force --profile release-army --bin codex
    RUSTY_V8_ARCHIVE=`{{ rusty_v8_script }} --archive` RUSTY_V8_SRC_BINDING_PATH=`{{ rusty_v8_script }} --binding` cargo install --path {{ rust_root }}/code-mode-host --locked --force --profile release-army --bin codex-code-mode-host

build-army-debug:
    RUSTY_V8_ARCHIVE=`{{ rusty_v8_script }} --archive` RUSTY_V8_SRC_BINDING_PATH=`{{ rusty_v8_script }} --binding` cargo build --locked --manifest-path {{ rust_root }}/Cargo.toml -p codex-cli --bin codex -p codex-code-mode-host --bin codex-code-mode-host

write-config-schema:
    cargo run --locked --manifest-path {{ rust_root }}/Cargo.toml -p codex-core --bin codex-write-config-schema

write-app-server-schema *args:
    cargo run --locked --manifest-path {{ rust_root }}/Cargo.toml -p codex-app-server-protocol --bin write_schema_fixtures -- {{args}}

write-hooks-schema:
    cargo run --locked --manifest-path {{ rust_root }}/Cargo.toml -p codex-hooks --bin write_hooks_schema_fixtures

log *args:
    cargo run --locked --manifest-path {{ rust_root }}/Cargo.toml -p codex-cli --bin logs_client -- {{args}}

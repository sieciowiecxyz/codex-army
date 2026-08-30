set positional-arguments := true

source_root := justfile_directory() / "codex-source"
rust_root := source_root / "codex-rs"
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
    cargo clippy --manifest-path {{ rust_root }}/Cargo.toml --fix --tests --allow-dirty {args}

clippy *args:
    cargo clippy --manifest-path {{ rust_root }}/Cargo.toml --tests {args}

install:
    rustup show active-toolchain
    cargo fetch --locked --manifest-path {{ rust_root }}/Cargo.toml

build-army:
    cargo build --locked --manifest-path {{ rust_root }}/Cargo.toml --profile release-army -p codex-cli --bin codex

install-army:
    cargo install --path {{ rust_root }}/cli --locked --force --profile release-army --bin codex

build-army-debug:
    cargo build --locked --manifest-path {{ rust_root }}/Cargo.toml -p codex-cli --bin codex

write-config-schema:
    cargo run --locked --manifest-path {{ rust_root }}/Cargo.toml -p codex-core --bin codex-write-config-schema

write-app-server-schema *args:
    cargo run --locked --manifest-path {{ rust_root }}/Cargo.toml -p codex-app-server-protocol --bin write_schema_fixtures -- {{args}}

write-hooks-schema:
    cargo run --locked --manifest-path {{ rust_root }}/Cargo.toml -p codex-hooks --bin write_hooks_schema_fixtures

log *args:
    cargo run --locked --manifest-path {{ rust_root }}/Cargo.toml -p codex-cli --bin logs_client -- {{args}}

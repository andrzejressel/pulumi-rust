set windows-shell := ["pwsh.exe", "-c"]

nightly := "nightly-2026-03-15"

install:
    rustup +{{nightly}} component add rustfmt
    rustup component add clippy
    rustup component add llvm-tools-preview
    mise install

check:
    cargo +{{nightly}} fmt -- --check
    cargo clippy --tests

fmt:
    cargo +{{nightly}} fmt
    cargo clippy --tests --fix --allow-dirty --allow-staged
    cd ast && just fmt
    cd pkg && just fmt

gen:
    cargo build
    rust2go-cli --src src/golang.rs --dst pkg/codegen/rust/api.go --package-name "rust" --without-main

test:
    cargo test
    cd ast && just test
    cd pkg && just test

regenerate-test $PULUMI_ACCEPT="1":
    cd ast && just regenerate-test PULUMI_ACCEPT={{PULUMI_ACCEPT}}
    cd pkg && just regenerate-test PULUMI_ACCEPT={{PULUMI_ACCEPT}}

# Generates files in `pkg/target` for easier introspection
test-local $LOCAL_TEST="1":
    just test

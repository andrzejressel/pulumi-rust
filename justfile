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
    cd pkg && go fmt ./...

gen:
    cargo build
    rust2go-cli --src src/golang.rs --dst pkg/codegen/rust/api.go --package-name "rust" --without-main

test:
    cd pkg && go test -v -count=1 ./...
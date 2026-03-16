set windows-shell := ["pwsh.exe", "-c"]

gen:
    cargo build
    ../rust2go/target/debug/rust2go-cli --src src/golang.rs --dst pkg/codegen/rust/api.go --package-name "rust" --without-main

test:
    cd pkg && go test -v -count=1 ./...
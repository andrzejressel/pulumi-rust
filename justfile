set windows-shell := ["pwsh.exe", "-c"]

gen:
    rust2go-cli --src src/golang.rs --dst pkg/rust/gen.go --without-main

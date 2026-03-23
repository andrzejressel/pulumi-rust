# Pulumi AST

This directory contains the Pulumi AST (Abstract Syntax Tree) module, which provides Go language support for working with Pulumi programs in an abstract syntax tree format.

## Overview

The AST module is a standalone Go module that can be used independently of the Rust-based Pulumi tooling. It provides:

- Protobuf definitions for Pulumi PCL (Pulumi Configuration Language) programs
- Code generation utilities for working with AST representations
- A language host implementation (`pulumi-language-ast`) for running Pulumi programs

## Usage

### As a Library

To use this module in your Go project:

```bash
go get github.com/andrzejressel/pulumi-rust/ast
```

### Building from Source

This module is part of a Go workspace. To build it:

```bash
# From the repository root
go build ./ast/...

# Or from within the ast directory
cd ast
go build ./...
```

### Running Tests

```bash
# From the repository root
go test ./ast/...

# Or from within the ast directory
cd ast
go test ./...
```

## Dependencies

This module depends on:
- `github.com/pulumi/pulumi/pkg/v3` - Pulumi core packages
- `github.com/pulumi/pulumi/sdk/v3` - Pulumi SDK

The repository includes the Pulumi source as a Git submodule in the `pulumi` directory at the repository root.

## Structure

- `cmd/pulumi-language-ast/` - The language host implementation
- `codegen/ast/` - AST code generation utilities
- `protobuf/` - Generated protobuf code
- `*.proto` - Protobuf definitions

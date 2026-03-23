# Pulumi AST

To regenerate go code from protobuf use: `just gen`

To regenerate tests after updating code use: `just regenerate-test`

To run tests use: `just test`

## Structure

The main things in this repo are proto schemas and mapping code.

- `package.proto` is for Pulumi Schema. The mapping code is in `codegen/ast/package.go`
- `pcl.proto` is for Pulumi Programs. The mapping code is in `codegen/ast/program.go`.
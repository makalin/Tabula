# Contributing to Tabula

Thank you for your interest in contributing to Tabula! This document provides guidelines and instructions for contributing.

## Development Setup

1. Clone the repository
2. Install Rust (latest stable version)
3. Build the project: `cargo build`
4. Run tests: `cargo test`

## Project Structure

- `/compiler` - Core compiler (lexer, parser, codegen)
- `/runtime` - Runtime system
- `/std` - Standard library
- `/lsp` - Language Server Protocol implementation
- `/tabpm` - Package manager
- `/repl` - Interactive REPL
- `/test-framework` - Testing framework
- `/debugger` - Debugger tool
- `/linter` - Code linter
- `/profiler` - Performance profiler
- `/docgen` - Documentation generator

## Coding Standards

- Use `cargo fmt` to format code
- Run `cargo clippy` before submitting PRs
- Write tests for new features
- Follow Rust naming conventions
- Document public APIs

## Submitting Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run `make test` and `make lint`
6. Submit a pull request

## Tools

- `tabula` - Main compiler
- `tabula-lsp` - Language server
- `tabpm` - Package manager
- `tabula-repl` - Interactive REPL
- `tabula-test` - Test runner
- `tabula-debug` - Debugger
- `tabula-lint` - Linter
- `tabula-profile` - Profiler
- `tabula-doc` - Documentation generator

## Questions?

Open an issue or start a discussion!


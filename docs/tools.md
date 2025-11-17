# Tabula Development Tools

Tabula comes with a comprehensive set of development tools to enhance your programming experience.

## Compiler (`tabula`)

The main compiler with multiple commands:

```bash
# Compile to native binary
tabula build -i program.tab -o program

# Compile to WebAssembly
tabula build -i program.tab -t wasm -o program.wasm

# Format code
tabula fmt -i program.tab --write

# Run program (interpreted)
tabula run -i program.tab
```

## Language Server (`tabula-lsp`)

Provides IDE integration with:
- Code completion
- Hover information
- Go to definition
- Document symbols
- Real-time diagnostics

Configure your editor to use `tabula-lsp` for the best experience.

## Package Manager (`tabpm`)

Manage dependencies and projects:

```bash
# Initialize new project
tabpm init my-project

# Add dependency
tabpm add http

# Install dependencies
tabpm install

# Publish package
tabpm publish
```

## REPL (`tabula-repl`)

Interactive development environment:

```bash
tabula-repl
```

Features:
- Real-time execution
- History tracking
- Syntax highlighting
- Error reporting

## Test Framework (`tabula-test`)

Run tests for your Tabula programs:

```bash
# Run all tests
tabula-test

# Run specific test
tabula-test --test mytest

# Verbose output
tabula-test --verbose
```

## Debugger (`tabula-debug`)

Debug Tabula programs with:
- Breakpoints
- Step-through execution
- Variable inspection
- Call stack viewing

```bash
tabula-debug program.tab
```

Controls:
- `s` - Step over
- `i` - Step into
- `c` - Continue
- `b` - Toggle breakpoint
- `q` - Quit

## Linter (`tabula-lint`)

Check code quality:

```bash
# Lint files
tabula-lint src/

# Auto-fix issues
tabula-lint --fix

# Verbose output
tabula-lint --verbose
```

Checks for:
- Mixed indentation
- Trailing whitespace
- Line length
- Naming conventions
- Syntax errors

## Profiler (`tabula-profile`)

Analyze performance:

```bash
# Profile program
tabula-profile program.tab

# JSON output
tabula-profile program.tab --format json -o profile.json

# Text output (default)
tabula-profile program.tab --format text
```

## Documentation Generator (`tabula-doc`)

Generate documentation:

```bash
# HTML documentation
tabula-doc src/ --format html --output docs/html

# Markdown documentation
tabula-doc src/ --format markdown --output docs/markdown
```

## Makefile Commands

Use the provided Makefile for common tasks:

```bash
make build    # Build all tools
make test     # Run tests
make fmt      # Format code
make lint     # Lint code
make clean    # Clean build artifacts
make install  # Install all tools
make docs     # Generate documentation
```


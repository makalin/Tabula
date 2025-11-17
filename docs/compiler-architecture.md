# Tabula Compiler Architecture

## Overview

The Tabula compiler is a multi-stage compiler that transforms Tabula source code into executable binaries or WebAssembly modules.

## Components

### 1. Lexer (`compiler/src/lexer/`)

The lexer tokenizes source code, recognizing:
- Whitespace tokens (TAB, SPACE, NEWLINE)
- Literals (NUMBER, FLOAT, STRING)
- Identifiers (WORD)
- Special tokens (EOF)

### 2. Parser (`compiler/src/parser/`)

The parser builds an Abstract Syntax Tree (AST) from tokens:
- Recognizes whitespace-based structure
- Handles TAB for blocks
- Handles SPACE for inline sequences
- Validates syntax

### 3. AST (`compiler/src/ast/`)

The AST represents the program structure:
- `Program` - root node
- `Statement` - various statement types
- `Expression` - expression tree
- Formatting support

### 4. Code Generator (`compiler/src/codegen/`)

Generates native code:
- LLVM IR generation (planned)
- C code generation (current)
- Interpreter for testing

### 5. WASM Generator (`compiler/src/wasm/`)

Generates WebAssembly:
- WAT (text format) generation
- WASM binary compilation
- Export/import handling

## Compilation Pipeline

```
Source Code → Lexer → Parser → AST → Codegen → Binary
                                    ↓
                                 WASM Gen → WASM
```

## Runtime

The runtime (`runtime/`) provides:
- Value representation
- Virtual machine (VM)
- Execution environment

## Standard Library

The standard library (`std/`) includes:
- IO operations
- String manipulation
- Collections
- HTTP client
- Async runtime
- Math functions


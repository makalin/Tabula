# Tabula  
### A whitespace-structured programming language where **tabs** define scope and **spaces** define flow.

Tabula is a modern compiled language that treats **whitespace as first-class syntax**.  
No braces. No semicolons. No visual noise.  
Your code becomes a clean, readable structure shaped entirely by indentation patterns.

---

## ✨ Why Tabula?

Tabula turns whitespace into logic:

- **TAB (`\t`) = Scope Operator**  
  Defines blocks, nesting, and structural hierarchy.

- **SPACE (` `) = Flow / Sequence Operator**  
  Controls inline grouping, argument passing, and chaining.

This creates an elegant syntax that reads like pseudocode but compiles like a systems language.

---

## 🧠 Core Concepts

### **Whitespace-First Syntax**
In Tabula, indentation *is the language*.  
The parser natively understands whitespace patterns as control structures.

### **Static + Shape Inference**
Strong typing like Rust, inference like Go.  
Minimal keywords, maximum clarity.

### **Single Codebase: Native + WASM**
Tabula compiles to:
- Native binaries (LLVM backend)
- WebAssembly for frontend logic

Perfect for full-stack systems.

---

## 🔣 Syntax Examples

### **Variables**
```

let name  "Mehmet"
let count  10

```

### **Functions**
```

func greet name
\tprint "Hello "  name

```

### **Conditionals**
```

if age > 30
\tprint "Adult"
else
\tprint "Young"

```

### **Loops**
```

for item in list
\tprocess item

```

### **Inline Sequence**
```

result  compute x  y   z

```

---

## 🧩 Language Rules (Summary)

- `TAB` starts a **block**
- `SPACE` controls **inline order, grouping, and tuples**
- `NEWLINE` ends statements
- Modules are folder-based (like Go)
- Compiler supports both AOT and JIT modes
- Built-in formatter enforces consistent whitespace patterns

---

## 🏗️ Project Goals

- Minimal, readable syntax  
- High performance  
- WASM-first design  
- Developer-friendly toolchain  
- Clean standard library for CLI, HTTP, async, FS, and math  
- Open-source from day one

---

## 📁 Repository Structure

```

/compiler
lexer/
parser/
ast/
codegen/
wasm/
/runtime
/std
/docs
/examples

```

---

## 🚧 Roadmap

### **Phase 1 — Grammar & Lexer**
- Whitespace tokenizer
- Core tokens: TAB, SPACE, WORD, NUMBER, STRING, NEWLINE, EOF

### **Phase 2 — Parser & AST**
- Block builder
- Inline operator analyzer
- Type inference engine

### **Phase 3 — Codegen**
- LLVM backend
- WASM backend
- Optional interpreter

### **Phase 4 — Standard Library**
- IO
- Strings
- Collections
- HTTP
- Async

### **Phase 5 — Tooling**
- Formatter (`tabfmt`)
- LSP server (`tabula-lsp`)
- Package manager (`tabpm`)

---

## 🧪 Example Program

```

func main
\tlet x  10
\tlet y  20
\tprint "Sum:"  x + y

```

---

## 📜 License
MIT (Open Source)

---

## 👤 Author
Mehmet T. AKALIN
https://github.com/makalin

---

## ⭐ Contribute
Issues, ideas, optimizations, PRs — all welcome.  
Tabula is a language built for clarity. Help shape its future.

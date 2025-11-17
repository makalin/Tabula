# Tabula Grammar Specification

## Tokens

- `TAB` (`\t`) - Scope/block delimiter
- `SPACE` (` `) - Inline sequence/flow operator
- `NEWLINE` (`\n`) - Statement terminator
- `WORD` - Identifier or keyword
- `NUMBER` - Integer literal
- `FLOAT` - Floating-point literal
- `STRING` - String literal (double-quoted)
- `EOF` - End of file

## Keywords

- `let` - Variable declaration
- `func` - Function definition
- `if` - Conditional statement
- `else` - Else clause
- `for` - Loop statement
- `in` - Iterator keyword
- `return` - Return statement
- `print` - Print statement

## Grammar Rules

### Program
```
program = statement*
```

### Statement
```
statement = let_stmt
          | func_stmt
          | if_stmt
          | for_stmt
          | print_stmt
          | return_stmt
          | expr_stmt

let_stmt = "let" SPACE WORD SPACE expr NEWLINE

func_stmt = "func" SPACE WORD (SPACE WORD)* NEWLINE
            TAB statement+

if_stmt = "if" SPACE expr NEWLINE
          TAB statement+
          ("else" NEWLINE TAB statement+)?

for_stmt = "for" SPACE WORD SPACE "in" SPACE expr NEWLINE
           TAB statement+

print_stmt = "print" SPACE expr (SPACE expr)* NEWLINE

return_stmt = "return" (SPACE expr)? NEWLINE

expr_stmt = expr NEWLINE
```

### Expression
```
expr = binary_expr

binary_expr = unary_expr (SPACE binary_op SPACE unary_expr)*

unary_expr = ("-" SPACE)? primary_expr

primary_expr = NUMBER
             | FLOAT
             | STRING
             | WORD
             | WORD SPACE expr (SPACE expr)*  // function call
```

### Binary Operators
```
binary_op = "+" | "-" | "*" | "/" | ">" | "<" | "=="
```

### Operator Precedence
1. `*`, `/`
2. `+`, `-`
3. `>`, `<`, `==`

## Whitespace Rules

1. **TAB** (`\t`) always starts a new block/scope
2. **SPACE** (` `) separates inline elements (arguments, operators, etc.)
3. **NEWLINE** (`\n`) terminates statements
4. Multiple spaces are treated as a single space
5. Tabs must be consistent (no mixing tabs and spaces for indentation)


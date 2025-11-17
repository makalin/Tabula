# Tabula Syntax Guide

## Variables

Declare variables with `let`:

```
let name  "Mehmet"
let count  10
let pi  3.14
```

## Functions

Define functions with `func`:

```
func greet name
	print "Hello "  name

func add a  b
	return a + b
```

Call functions with space-separated arguments:

```
greet "World"
let result  add 10  20
```

## Conditionals

Use `if` and `else`:

```
if age > 30
	print "Adult"
else
	print "Young"
```

## Loops

Iterate with `for`:

```
for item in list
	process item
```

## Expressions

Binary operations use spaces:

```
let sum  x + y
let product  a * b
let is_greater  x > y
```

## Inline Sequences

Multiple operations on one line:

```
result  compute x  y   z
```

This calls `compute` with arguments `x`, `y`, and `z`.

## Best Practices

1. Use tabs consistently for indentation
2. Use spaces for inline separators
3. Keep function bodies indented with one tab
4. Use meaningful variable and function names
5. Format code with `tabula fmt`


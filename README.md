[![progress-banner](https://backend.codecrafters.io/progress/interpreter/6487f0ca-ecde-4403-9796-aa885310514b)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

# Lox Interpreter in Rust

This is a Rust implementation of the Lox language interpreter for the
["Build Your Own Interpreter" Challenge](https://app.codecrafters.io/courses/interpreter/overview) on CodeCrafters.

## Features

This interpreter supports a rich set of language features:

- **Expressions**: Arithmetic operations, comparisons, logical operations
- **Variables**: Declaration and assignment
- **Control Flow**: If statements, while loops, for loops
- **Functions**: Declaration, calling, and returning values
- **Closures**: Functions can capture variables from their enclosing scope
- **Classes**: Object-oriented programming with class declarations
- **Inheritance**: Classes can inherit from another class using the `<` operator
- **Methods**: Classes can have methods including special `init` constructor
- **Native Functions**: Built-in functions like `clock()`

## Getting Started

1. Ensure you have `cargo (1.94+)` installed locally
2. Run `./your_program.sh run <filename.lox>` to interpret a Lox script

## Usage Examples

The interpreter supports several modes:

```sh
# Run a full Lox program
./your_program.sh run test/constructor_calls/3.lox

# Just tokenize the file to see lexical analysis
./your_program.sh tokenize test/constructor_calls/3.lox

# Just evaluate an expression
./your_program.sh evaluate test/string_concat.lox

# Just parse the expression and output the AST
./your_program.sh parse test/string_concat.lox
```

## Lox Language Examples

### Variables and Control Flow

```js
var a = 1;
var b = 2;
if (a < b) {
  print "a is less than b";
} else {
  print "a is greater than or equal to b";
}
```

### Functions

```js
fun fibonacci(n) {
  if (n <= 1) return n;
  return fibonacci(n - 2) + fibonacci(n - 1);
}

print fibonacci(10);
```

### Classes and Inheritance

```js
class Animal {
  init(name) {
    this.name = name;
  }

  speak() {
    print this.name + " makes a noise";
  }
}

class Dog < Animal {
  speak() {
    print this.name + " barks";
  }
}

var dog = Dog("Rex");
dog.speak();  // Prints: Rex barks
```

### Higher-Order Functions

```js
fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    return i;
  }
  return count;
}

var counter = makeCounter();
print counter(); // 1
print counter(); // 2
```

## Architecture

The interpreter is structured in these key components:

- **Scanner (`scanner.rs`)**: Tokenizes the source code into lexical tokens
- **Parser (`parser.rs`)**: Builds an abstract syntax tree from tokens
- **Resolver (`resolver.rs`)**: Performs variable resolution and semantic analysis
- **Interpreter (`interpreter.rs`)**: Executes the resolved syntax tree
- **Environment (`environment.rs`)**: Manages variable scopes and lookups
- **Callable (`callable.rs`)**: Implements functions, classes, and method calls
- **Grammar (`grammar.rs`)**: Defines the AST node types and tokens

## Implementation Details

The interpreter follows a classic compilation pipeline:

1. **Scanning**: Converts source code into tokens
2. **Parsing**: Transforms tokens into an Abstract Syntax Tree
3. **Resolution**: Resolves variable references and performs static analysis
4. **Interpretation**: Executes the program

### Error Handling

The interpreter implements comprehensive error handling:

- **Lexical errors**: Detected during scanning
- **Syntax errors**: Reported during parsing
- **Semantic errors**: Caught during resolution (e.g., undefined variables)
- **Runtime errors**: Thrown during execution (e.g., type errors, division by zero)

Each error provides meaningful context with line numbers and error messages.

### Error Recovery

The parser and resolver both implement error recovery to report multiple errors per run:

- **Parser**: On error, synchronizes to the next statement boundary (`;` or keyword) and continues. Error recovery works at both the top level and inside blocks/functions.
- **Resolver**: Collects per-statement errors and continues resolving remaining statements. Scope stack is truncated on error to maintain consistency.

## Performance Considerations

Memory management is optimized to minimize heap allocations on hot paths:

- **`Rc<str>` for strings**: Token lexemes (`Token.lexeme`) and string values (`Literal::String`) use `Rc<str>`. Variable lookups, assignments, and function calls clone a refcount — no heap allocation.
- **`Rc<Function>` for AST sharing**: Function declarations are wrapped in `Rc` at parse time. Defining closures and binding methods (`bind()`) bump a refcount — no deep-clone of the AST body.
- **Byte-based scanner**: The scanner operates on `&[u8]` (1 byte/char). Substring extraction slices directly into the source buffer.
- **Zero-copy token output**: `scan_tokens()` uses `std::mem::take` to move the token vec out — no per-token cloning.
- **`Rc<str>` HashMap keys**: Environment variable maps and class method/field maps use `Rc<str>` keys, reusing the token's lexeme without allocating new strings.

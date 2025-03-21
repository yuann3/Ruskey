![CleanShot 2025-03-08 at 16 57 34@2x](https://github.com/user-attachments/assets/92e15a16-dcc9-45b3-9c5a-93fc98e06b58)

A implementation of the Monkey programming language interpreter in Rust, based on the book "Writing An Interpreter In Go" by Thorsten Ball.

## About

Ruskey is Rust implementation of the Monkey programming language introduced in the book "Writing An Interpreter In Go". This project serves as both a learning exercise for Rust and interpreter design.

The Monkey language features:

- C-like syntax
- Variable bindings
- Integer and boolean data types
- Arithmetic expressions
- Built-in functions
- First-class and higher-order functions
- Closures
- String data structure
- Array data structure
- Hash data structure

## Project Structure

```
ruskey/
├── src/
│   ├── token.rs       # Token definitions
│   ├── lexer.rs       # Lexical analyzer
│   ├── ast.rs         # Abstract Syntax Tree
│   ├── parser.rs      # Parser
│   ├── object.rs      # Object system
│   ├── environment.rs # Environment for variable bindings
│   ├── evaluator.rs   # AST evaluator
│   ├── repl.rs        # Read-Eval-Print Loop
│   └── lib.rs         # Library exports
├── tests/             # Test suite
└── Cargo.toml         # Project configuration
```

## Features

- **Lexer**: Tokenizes Monkey source code
- **Parser**: Recursive descent parser with Pratt parsing for expressions
- **AST**: Represents the structure of Monkey programs
- **Evaluator**: Interprets and evaluates Monkey code
- **Object System**: Represents values and objects in Monkey
- **Environment**: Tracks variable bindings and scopes
- **REPL**: Interactive shell for experimenting with Monkey

## Progress

- [x] Lexer implementation
- [x] Parser implementation
- [x] AST evaluator
- [x] Object system
- [x] Environment
- [x] Function evaluation
- [x] REPL

## Building and Running

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run the REPL
cargo run
```

## Example Code

```
// Define a function
let fibonacci = fn(x) {
  if (x == 0) {
    return 0;
  } else {
    if (x == 1) {
      return 1;
    } else {
      return fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
};

// Call the function
fibonacci(10);
```

## Learning Resources

If you're interested in learning more about interpreters or following along with this project:

1. ["Writing An Interpreter In Go"](https://interpreterbook.com/) by Thorsten Ball
2. ["Crafting Interpreters"](https://craftinginterpreters.com/) by Bob Nystrom

## License

This project is open source and available under the [MIT License](LICENSE).

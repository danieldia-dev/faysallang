# faysallang
highkey the best language on the planet, written in Rust.

## Syntax Overview

### Keywords

- `hayde` - Declare a variable (like "let" or "var")
- `hiyye` - Assignment operator (like "=")
- `ong_no_cap` - Boolean true (means "for real, no lies")
- `cap` - Boolean false (means "lie")
- `eza` - If statement (means "if")
- `betshil` - Condition check (means "check if")
- `lakan` - Then (block start)
- `deal` - End block
- `3mol` - Do/make command
- `highkey` - Emphasis modifier for print

## Example Programs

### Basic Variable and Conditional

```faysallang
hayde x hiyye ong_no_cap
eza betshil x lakan
   3mol highkey "x is fr ong no cap"
deal
```

This code:
1. Declares a variable `x` and assigns it `true`
2. Checks if `x` is true
3. If true, prints "x is fr ong no cap"

### More Examples

```faysallang
hayde name hiyye "Faysal"
3mol highkey "Hello there"

hayde age hiyye 25
hayde is_cool hiyye ong_no_cap

eza betshil is_cool lakan
   3mol highkey "You're highkey cool"
deal
```

## Building and Running

```bash
# Build the project
cargo build --release

# Run the interpreter
cargo run

# Or run the compiled binary
./target/release/faysallang
```

## Implementation Details

The interpreter consists of three main components:

1. **Lexer** - Tokenizes the source code into tokens
2. **Parser** - Builds an Abstract Syntax Tree (AST) from tokens
3. **Interpreter** - Executes the AST

### Current Features

- Variable declarations
- Boolean values (ong_no_cap/cap)
- String literals
- Number literals
- If statements with conditions
- Print statements
- Basic identifier lookup

### Planned Features

- Arithmetic operations
- Comparison operators
- While loops
- Functions
- Arrays/lists
- More complex expressions

## Language Philosophy

Faysal Lang combines modern slang with programming concepts, making code feel more conversational while maintaining logical structure. The syntax draws from Gen-Z vernacular mixed with Arabic-influenced terms, creating a unique programming experience.

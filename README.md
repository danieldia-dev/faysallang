# Faysal Lang 🔥
Highkey the best fully functional programming language on the planet with slang-inspired syntax, written in Rust. Features arithmetic, logic, control flow, loops, and more!

## Features ✨

### Full Feature List

- **Variables** - Declare and assign values
- **Data Types** - Numbers, strings, booleans
- **Arithmetic** - Add, subtract, multiply, divide, modulo
- **Comparisons** - ==, !=, <, >, <=, >=
- **Logic** - AND (&&), OR (||), NOT (!)
- **Control Flow** - If/else statements
- **Loops** - While loops with break/continue
- **Printing** - Output to console
- **Comments** - Single line with //

## Syntax Overview

### Keywords

**Variables & Values:**
- `hayde` - Declare a variable (like "let" or "var")
- `hiyye` - Assignment operator (like "=")
- `ong_no_cap` - Boolean true (means "for real, no lies")
- `cap` - Boolean false (means "lie")

**Control Flow:**
- `eza` - If statement (means "if")
- `betshil` - Condition check (means "check if")
- `lakan` - Then (block start)
- `walla` - Else
- `deal` - End block

**Loops:**
- `khalas` - While loop (means "done/finished when...")
- `yalla` - Continue (means "let's go!")
- `waqif` - Break (means "stop!")

**I/O:**
- `3mol` - Do/make command (for printing)
- `highkey` - Emphasis modifier for print
- `lowkey` - Debug output (to stderr)

**Operators:**
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `&&`, `||`, `!`
- Parentheses: `(`, `)` for grouping

## Example Programs

### 1. Basic Arithmetic

```faysallang
hayde x hiyye 10
hayde y hiyye 5

hayde sum hiyye x + y
3mol highkey sum          // prints: 15

hayde product hiyye x * y
3mol highkey product      // prints: 50
```

### 2. Conditionals

```faysallang
hayde age hiyye 21

eza betshil age >= 18 lakan
   3mol highkey "You can vote!"
walla
   3mol highkey "Too young to vote"
deal
```

### 3. While Loop (Counting)

```faysallang
hayde counter hiyye 1

khalas betshil counter <= 5 lakan
   3mol highkey counter
   counter hiyye counter + 1
deal

// prints: 1 2 3 4 5
```

### 4. FizzBuzz

```faysallang
hayde num hiyye 1

khalas betshil num <= 15 lakan
   hayde by_three hiyye num % 3
   hayde by_five hiyye num % 5
   
   eza betshil by_three == 0 && by_five == 0 lakan
      3mol highkey "FizzBuzz"
   walla
      eza betshil by_three == 0 lakan
         3mol highkey "Fizz"
      walla
         eza betshil by_five == 0 lakan
            3mol highkey "Buzz"
         walla
            3mol highkey num
         deal
      deal
   deal
   
   num hiyye num + 1
deal
```

### 5. Factorial Calculator

```faysallang
hayde n hiyye 5
hayde factorial hiyye 1
hayde i hiyye 1

khalas betshil i <= n lakan
   factorial hiyye factorial * i
   i hiyye i + 1
deal

3mol highkey factorial  // prints: 120
```

### 6. Prime Number Check

```faysallang
hayde num hiyye 17
hayde is_prime hiyye ong_no_cap
hayde divisor hiyye 2

khalas betshil divisor * divisor <= num lakan
   hayde remainder hiyye num % divisor
   eza betshil remainder == 0 lakan
      is_prime hiyye cap
      waqif
   deal
   divisor hiyye divisor + 1
deal

eza betshil is_prime lakan
   3mol highkey "It's prime!"
deal
```

### 7. Break and Continue

```faysallang
hayde i hiyye 0

khalas betshil i < 10 lakan
   i hiyye i + 1
   
   // Skip even numbers
   eza betshil i % 2 == 0 lakan
      yalla
   deal
   
   // Stop at 7
   eza betshil i > 7 lakan
      waqif
   deal
   
   3mol highkey i
deal

// prints: 1 3 5 7
```

### 8. String Concatenation

```faysallang
hayde first hiyye "Hello "
hayde second hiyye "World"
hayde message hiyye first + second
3mol highkey message  // prints: Hello World
```

## Building and Running

### Build the Project

```bash
cargo build --release
```

### Run a File

```bash
# Run a Faysal Lang file
cargo run examples/test.fsl

# Or use the compiled binary
./target/release/faysallang examples/test.fsl
```

### Interactive REPL

```bash
# Start the REPL
cargo run

# Then type commands interactively
faysal> hayde x hiyye 42
faysal> 3mol highkey x
42
faysal> exit
```

## Implementation Details

The interpreter consists of three main components:

1. **Lexer** - Tokenizes source code into tokens
2. **Parser** - Builds an Abstract Syntax Tree (AST) with proper operator precedence
3. **Interpreter** - Executes the AST with variable storage and control flow

### Technical Features

- **Operator Precedence** - Properly handles mathematical order of operations
- **Type Coercion** - Automatic conversion between numbers, strings, and booleans
- **Truthiness** - Non-zero numbers and non-empty strings are truthy
- **Error Handling** - Division by zero returns infinity
- **Comments** - Single-line comments with `//`
- **REPL Mode** - Interactive shell for testing

## Examples Directory

Check out the `examples/` directory for more:

- `test.fsl` - Comprehensive feature demonstration
- `game.fsl` - Simple number guessing game

## Language Philosophy

Faysal Lang combines modern slang with programming concepts, making code feel more conversational while maintaining logical structure. The syntax draws from Gen-Z vernacular mixed with Arabic-influenced terms (like "yalla", "khalas", "walla"), creating a unique and expressive programming experience.

### Why "ong_no_cap"?

In internet slang:
- "ong" = "on God" (I swear)
- "no cap" = "no lie" / "for real"
- Together = "absolutely true, no lies" = perfect for `true`!

### Why "3mol"?

- "3" in Arabic chat = "ع" sound
- "3mol" = "عمل" = "do/make/perform"
- Perfect for executing commands like printing!

## Future Features

- [ ] Functions and procedures
- [ ] Arrays and lists
- [ ] Hash maps / dictionaries
- [ ] String interpolation
- [ ] File I/O
- [ ] Import/module system
- [ ] Better error messages with line numbers
- [ ] Standard library functions
- [ ] Random number generation

## Contributing

Feel free to contribute! Some ideas:
- Add more examples
- Improve error messages
- Add new features
- Write tests
- Improve documentation

## License

MIT License - Feel free to vibe with this code however you want! 🚀

---

**Made with 💯 by the Faysal Lang team**

*No cap, this language is actually fr fr* 🔥
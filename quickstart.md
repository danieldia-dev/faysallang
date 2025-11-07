# Faysal Lang - Quick Start Tutorial 🚀

Welcome to Faysal Lang! This tutorial will get you coding in 5 minutes.

## Hello World

Let's start simple:

```faysallang
3mol highkey "Hello World!"
```

That's it! `3mol highkey` means "print" in Faysal Lang.

## Variables

Declare variables with `hayde` and assign with `hiyye`:

```faysallang
hayde name hiyye "Faysal"
hayde age hiyye 25
hayde is_cool hiyye ong_no_cap

3mol highkey name
3mol highkey age
3mol highkey is_cool
```

### Data Types

- **Numbers**: `42`, `3.14`, `-10`
- **Strings**: `"hello"`, `"what's good"`
- **Booleans**: `ong_no_cap` (true), `cap` (false)

## Math Time 🧮

```faysallang
hayde a hiyye 10
hayde b hiyye 3

3mol highkey a + b   // Addition: 13
3mol highkey a - b   // Subtraction: 7
3mol highkey a * b   // Multiplication: 30
3mol highkey a / b   // Division: 3.33...
3mol highkey a % b   // Modulo (remainder): 1
```

## Making Decisions 🤔

Use `eza` for if statements:

```faysallang
hayde age hiyye 21

eza betshil age >= 18 lakan
   3mol highkey "Adult status unlocked!"
walla
   3mol highkey "Still a youngin"
deal
```

Keywords:
- `eza` = if
- `betshil` = check (optional, for readability)
- `lakan` = then
- `walla` = else
- `deal` = end

## Comparisons

```faysallang
hayde x hiyye 10
hayde y hiyye 20

eza betshil x == y lakan
   3mol highkey "Equal"
deal

eza betshil x < y lakan
   3mol highkey "x is smaller"
deal

// Available: ==, !=, <, >, <=, >=
```

## Logic Gates 🚪

```faysallang
hayde hungry hiyye ong_no_cap
hayde has_food hiyye cap

// AND
eza betshil hungry && has_food lakan
   3mol highkey "Time to eat!"
deal

// OR
eza betshil hungry || has_food lakan
   3mol highkey "Food situation detected"
deal

// NOT
eza betshil !has_food lakan
   3mol highkey "Need to get food"
deal
```

## Loops 🔄

While loops with `khalas`:

```faysallang
hayde count hiyye 1

khalas betshil count <= 5 lakan
   3mol highkey count
   count hiyye count + 1
deal
```

### Loop Control

Skip iterations with `yalla` (continue):

```faysallang
hayde i hiyye 0

khalas betshil i < 10 lakan
   i hiyye i + 1

   eza betshil i % 2 == 0 lakan
      yalla  // Skip even numbers
   deal

   3mol highkey i
deal
```

Exit early with `waqif` (break):

```faysallang
hayde i hiyye 0

khalas betshil i < 100 lakan
   i hiyye i + 1

   eza betshil i == 5 lakan
      waqif  // Stop at 5
   deal

   3mol highkey i
deal
```

## Your First Real Program 💪

Let's calculate if a number is even or odd:

```faysallang
hayde number hiyye 42
hayde remainder hiyye number % 2

eza betshil remainder == 0 lakan
   3mol highkey "Even number!"
walla
   3mol highkey "Odd number!"
deal
```

## Challenge: Countdown ⏱️

Try writing a countdown from 10 to 1:

<details>
<summary>Solution (click to reveal)</summary>

```faysallang
hayde count hiyye 10

khalas betshil count > 0 lakan
   3mol highkey count
   count hiyye count - 1
deal

3mol highkey "Blastoff!"
```
</details>

## Challenge: Sum of Numbers 🔢

Calculate the sum of numbers 1 through 10:

<details>
<summary>Solution (click to reveal)</summary>

```faysallang
hayde sum hiyye 0
hayde i hiyye 1

khalas betshil i <= 10 lakan
   sum hiyye sum + i
   i hiyye i + 1
deal

3mol highkey sum  // Should print 55
```
</details>

## Running Your Code

### Option 1: Save to a file

```bash
# Save your code as program.fsl
# Then run:
cargo run program.fsl
```

### Option 2: Use the REPL

```bash
# Start interactive mode:
cargo run

# Type your code line by line:
faysal> hayde x hiyye 42
faysal> 3mol highkey x
42
faysal> exit
```

## Common Patterns

### Counter Pattern
```faysallang
hayde i hiyye 0
khalas betshil i < 10 lakan
   // do something
   i hiyye i + 1
deal
```

### Accumulator Pattern
```faysallang
hayde total hiyye 0
hayde i hiyye 1
khalas betshil i <= 100 lakan
   total hiyye total + i
   i hiyye i + 1
deal
```

### Find Pattern
```faysallang
hayde found hiyye cap
hayde i hiyye 0
khalas betshil i < 10 && !found lakan
   eza betshil i == 5 lakan
      found hiyye ong_no_cap
   deal
   i hiyye i + 1
deal
```

## Tips & Tricks 💡

1. **Parentheses**: Use `()` to group expressions:
   ```faysallang
   hayde result hiyye (10 + 5) * 2
   ```

2. **Comments**: Use `//` for notes:
   ```faysallang
   // This is a comment
   hayde x hiyye 42  // Comments can go here too
   ```

3. **String concatenation**: Use `+` to join strings:
   ```faysallang
   hayde greeting hiyye "Hello " + "World"
   ```

4. **Truthiness**:
   - Numbers: 0 is falsy, everything else is truthy
   - Strings: empty `""` is falsy, anything else is truthy
   - Booleans: obvious!

## Next Steps 🎯

1. Check out `examples/test.fsl` for comprehensive examples
2. Try `examples/game.fsl` to see a game in action
3. Build something cool and share it!
4. Read the full README for advanced features

## Need Help? 🆘

- Syntax not working? Check for typos in keywords
- Loop running forever? Make sure your condition eventually becomes false
- Variable not found? Make sure you declared it with `hayde` first

## You're Ready! 🎉

You now know enough to build:
- Calculators
- Games
- Math solvers
- Text processors
- Logic puzzles

Now go build something dope! No cap! 🔥

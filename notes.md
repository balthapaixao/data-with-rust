## [Basic Syntax](https://datawithrust.com/chapter_2/chapter_2_4.html#basic-syntax)

### Variables

- Variables are immutable by default;
- To make a variable mutable, use the `mut` keyword;
- Difference between `let`, `const` and `static`:
  - `let` is used to declare a variable;
  - `const` is used to declare a constant;
  - `static` is used to declare a global variable. It can be mutated but with some conditions (using unsafe code blocks).
- Variable shadowing:
  - You can declare a new variable with the same name as a previous variable;
  - The new variable shadows the previous variable;
  - The new variable can have a different type than the previous variable;
  - The new variable can be mutable, even if the previous variable was immutable;
  - The new variable can be declared in a different scope than the previous variable.

### Conditional Structure

#### if/else

More appropriate for simple boolean conditions

- The parenthesis are not mandatory, but I strongly recommend them. Explicit is better than implicit.

#### match

More appropriate for complex conditions that require pattern matching (matching types for example) and multiple possible outcomes

- The `match` expression is exhaustive, which means that you must cover all possible cases;
- The `_` placeholder can be used to match all remaining cases;

### Loops

Three ways to loop in Rust: `loop`, `while` and `for`.

#### loop

- The `loop` keyword is used to create an infinite loop;
- Same thing as `while true`;

#### while

- The `while` keyword is used to create a loop that runs while a condition is true;
- The condition must be a boolean expression;
- The `while` loop is not very common in Rust;

#### for

- The `for` keyword is used to create a loop that runs for each item in a collection;
- The `for` loop is the most common loop in Rust;

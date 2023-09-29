# Data with Rust Notes

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

### Data Types

#### Scalar types

##### Integer Types

- Signed integers: `i8`, `i16`, `i32`, `i64`, `i128` and `isize`;
- Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128` and `usize`;
- The number after the type represents the number of bits used to store the integer;
- The `isize` and `usize` types depend on the architecture of the machine that is running the program;

##### Floats Types

- `f32` (single-precision) and `f64` (double-precision);
- The `f64` type is the default;
- The `f32` type is faster than `f64` on some architectures;
- The `f64` type is more precise than `f32`;

##### Boolean Type

- `bool` type;
- `true` and `false` values;
- 1 byte in size;

##### Character Type

- `char` type;
- 4 bytes in size;
- Unicode scalar values (4 bytes each);
- Single quotes (`'`) are used to declare a `char` type;
- ASCII characters are `char` types;

##### String Type

- Two main types:

  - String (heap allocated and growable);
  - &str (string slice, immutable);

- `str` type;
- Double quotes (`"`) are used to declare a `str` type;
- The `String` type is a growable, mutable, owned, UTF-8 encoded string type;
- The `String` type is provided by the standard library, not the core language;
- The `String` type is implemented as a struct that contains three fields:
  - A pointer to the memory that holds the contents of the string;
  - A length;
  - A capacity;

#### Compound types

##### Tuple Type

- A tuple is a collection of values of different types;
- Tuples have a fixed length: once declared, they cannot grow or shrink in size;
- Tuples are declared using parenthesis;
- Similar to tuples in Python and SQL;
- Tuples are useful when you want to express a fixed set of values;

##### Array Type

- An array is a collection of values of the same type;
- Arrays have a fixed length: once declared, they cannot grow or shrink in size;
- Arrays are declared using square brackets;

#### Custom Data Types

Rust allows you to create custom data types using struct, enum and union.

##### Structs

- A struct is a custom data type that allows you to name and package together multiple related values that make up a meaningful group;
- Structs are similar to tuples, but with named fields;
- Structs are declared using the `struct` keyword;

##### Enums

- An enum is a custom data type that allows you to define a type by enumerating its possible variants;
- Enums are similar to structs, but with named variants;
- Enums are declared using the `enum` keyword;
- very useful for serializing and deserializing data;
- Can be generic;

##### Unions

-

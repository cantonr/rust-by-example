# rust-by-example

Learning Rust at https://doc.rust-lang.org/stable/rust-by-example

## Hello World

- println! is a [macro](https://doc.rust-lang.org/stable/rust-by-example/macros.html) that prints text to the console. 
  
- A binary can be generated using the Rust compiler: rustc.

```shell
$ rustc _1-hello.rs
```

- rustc will produce a hello binary that can be executed.

```shell
$ ./_1-hello
Hello World!
```

### Comments

Any program requires comments, and Rust supports a few different varieties:

- Regular comments which are ignored by the compiler:
    - `// Line comments which go to the end of the line.`
    - `/* Block comments which go to the closing delimiter. */`
- Doc comments which are parsed into HTML library documentation:
    - `/// Generate library docs for the following item.`
    - `//! Generate library docs for the enclosing item.`

See also:

[Library documentation](https://doc.rust-lang.org/stable/rust-by-example/meta/doc.html)

### Formatted Print

Printing is handled by a series of macros defined in std::fmt some of which include:

- `format!`: write formatted text to `String`
- `print!`: same as `format!` but the text is printed to the console (io::stdout).
- `println!`: same as `print!` but a newline is appended.
- `eprint!`: same as `format!` but the text is printed to the standard error (io::stderr).
- `eprintln!`: same as `eprint!` but a newline is appended.

All parse text in the same fashion. As a plus, Rust checks formatting correctness at compile time.

## Primitives

Rust provides access to a wide variety of primitives. A sample includes:

### Scalar Types

- signed integers: `i8`, `i16`, `i32`, `i64`, `i128` and `isize` (pointer size)
- unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128` and `usize` (pointer size)
- floating point: `f32`, `f64`
- char Unicode scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each)
- bool either `true` or `false`
- and the unit type (), whose only possible value is an empty tuple: ()

Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

###Compound Types

- arrays like `[1, 2, 3]`
- tuples like `(1, true)`

### Literals and operators

Integers `1`, floats `1.2`, characters `'a'`, strings `"abc"`, booleans `true` and the unit type () can be expressed using literals.

Integers can, alternatively, be expressed using hexadecimal, octal or binary notation using these prefixes respectively: `0x`, `0o` or `0b`.

Underscores can be inserted in numeric literals to improve readability, e.g. `1_000` is the same as `1000`, and `0.000_001` is the same as `0.000001`.

We need to tell the compiler the type of the literals we use. For now, we'll use the `u32` suffix to indicate that the literal is an unsigned 32-bit integer, and the `i32` suffix to indicate that it's a signed 32-bit integer.

The operators available and their precedence in Rust are similar to other [C-like languages](https://en.wikipedia.org/wiki/Operator_precedence#Programming_languages).

### Tuples

A tuple is a collection of values of different types. Tuples are constructed using parentheses (), and each tuple itself is a value with type signature `(T1, T2, ...)`, where `T1`, `T2` are the types of its members. Functions can use tuples to return multiple values, as tuples can hold any number of values.

### Arrays and Slices

An array is a collection of objects of the same type `T`, stored in contiguous memory. Arrays are created using brackets `[]`, and their length, which is known at compile time, is part of their type signature `[T; length]`.

Slices are similar to arrays, but their length is not known at compile time. Instead, a slice is a two-word object, the first word is a pointer to the data, and the second word is the length of the slice. The word size is the same as usize, determined by the processor architecture eg 64 bits on an x86-64. Slices can be used to borrow a section of an array, and have the type signature `&[T]`.

## Custom Types

Rust custom data types are formed mainly through the two keywords:

- `struct`: define a structure
- `enum`: define an enumeration

Constants can also be created via the const and static keywords.

### Structures

There are three types of structures ("structs") that can be created using the `struct` keyword:

- Tuple structs, which are, basically, named tuples.
- The classic [C structs](https://en.wikipedia.org/wiki/Struct_(C_programming_language))
- Unit structs, which are field-less, are useful for generics.


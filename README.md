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

### Enums

The `enum` keyword allows the creation of a type which may be one of a few different variants. Any variant which is valid as a `struct` is also valid as an `enum`.

#### Type aliases

If you use a type alias, you can refer to each enum variant via its alias. This might be useful if the enum's name is too long or too generic, and you want to rename it.

### constants

Rust has two different types of constants which can be declared in any scope including global. Both require explicit type annotation:

- `const`: An unchangeable value (the common case).
- `static`: A possibly `mut`able variable with [`'static`](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html) lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is [`unsafe`](https://doc.rust-lang.org/rust-by-example/unsafe.html).

## Variable Bindings

Rust provides type safety via static typing. Variable bindings can be type annotated when declared. However, in most cases, the compiler will be able to infer the type of the variable from the context, heavily reducing the annotation burden.

Values (like literals) can be bound to variables, using the let binding.

### Mutability

Variable bindings are immutable by default, but this can be overridden using the `mut` modifier.

The compiler will throw a detailed diagnostic about mutability errors.

### Scope and Shadowing

Variable bindings have a scope, and are constrained to live in a block. A block is a collection of statements enclosed by braces `{}`. 

### Declare first

It's possible to declare variable bindings first, and initialize them later. However, this form is seldom used, as it may lead to the use of uninitialized variables.

The compiler forbids use of uninitialized variables, as this would lead to undefined behavior.

### Freezing

When data is bound by the same name immutably, it also _freezes_. Frozen data can't be modified until the immutable binding goes out of scope

## Types

Rust provides several mechanisms to change or define the type of primitive and user defined types. The following sections cover:

- [Casting](https://doc.rust-lang.org/rust-by-example/types/cast.html) between primitive types
- Specifying the desired type of [literals](https://doc.rust-lang.org/rust-by-example/types/literals.html)
- Using [type inference](https://doc.rust-lang.org/rust-by-example/types/inference.html)
- [Aliasing](https://doc.rust-lang.org/rust-by-example/types/alias.html) types

### Casting

Rust provides no implicit type conversion (coercion) between primitive types. But, explicit type conversion (casting) can be performed using the `as` keyword.

Rules for converting between integral types follow C conventions generally, except in cases where C has undefined behavior. The behavior of all casts between integral types is well defined in Rust.

### Literals

Numeric literals can be type annotated by adding the type as a suffix. As an example, to specify that the literal `42` should have the type `i32`, write `42i32`.

The type of unsuffixed numeric literals will depend on how they are used. If no constraint exists, the compiler will use `i32` for integers, and `f64` for floating-point numbers.

### Inference

The type inference engine is pretty smart. It does more than looking at the type of the value expression during an initialization. It also looks at how the variable is used afterwards to infer its type. Here's an advanced example of type inference:

```rust
fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}
```

### Aliasing

The `type` statement can be used to give a new name to an existing type. Types must have `UpperCamelCase` names, or the compiler will raise a warning. The exception to this rule are the primitive types: `usize`, `f32`, etc.

The main use of aliases is to reduce boilerplate; for example the `IoResult<T>` type is an alias for the `Result<T, IoError>` type.
# rust-by-example

Learning Rust at https://doc.rust-lang.org/stable/rust-by-example

## Hello World

- println! is a [macro](https://doc.rust-lang.org/stable/rust-by-example/macros.html) that prints text to the console. 
  
- A binary can be generated using the Rust compiler: rustc.

```shell
$ rustc hello.rs
```

- rustc will produce a hello binary that can be executed.

```shell
$ ./hello
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
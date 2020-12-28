# S-expression parser

Simplified S-expression parser written in rust for a [coding interview]([https://link](https://gist.github.com/rraval/2ef5e2ff228e022653db2055fc12ea9d))

# Example Usage 

```
s-expression "(multiply (add 1 2) 3)"
```

# Assumptions

- all input is well formed 
  - Parenthesis will always be balanced.
  - Only the add and multiply functions will be called.
  - There will always be a single space between the function arguments.
- only natural numbers are used and not larger than an `i64`

# Building
Requires the [rustup](https://rustup.rs/) tool chain and cargo


# Libraries used

## [logos](https://github.com/maciejhirsz/logos)

logos is a macro library for rust that can build fast and efficient lexers

## [rowan](https://github.com/rust-analyzer/rowan)

rowan is the internal library used to build [rust-analyzer](h[ttps://link](https://github.com/rust-analyzer/rust-analyzer)) is support abstract syntax tree parsing and evaluation

## [structopt](https://github.com/TeXitoi/structopt)
structopt is a macro library to build cli arguments using macros and structs

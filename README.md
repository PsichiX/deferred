# Deferred
Rust crate to help perform deferred execution of code logic.

![Travis CI](https://travis-ci.org/PsichiX/deferred.svg?branch=master)
[![Docs.rs](https://docs.rs/deferred/badge.svg)](https://docs.rs/deferred)
[![Crates.io](https://img.shields.io/crates/v/deferred.svg)](https://crates.io/crates/deferred)

# Idea
This crate can be used specifically for operations where you need to execute
different parts of logic at unspecified time but you cannot use futures or any
other asynchronous operations.

# Usage
Record in `Cargo.toml`:
```toml
[dependencies]
deferred = "1.0.0"
```

Your crate module:
```rust
use deferred::Deferred;

let context = Deferred::new(1, vec![
    |v| v + 1,
    |v| v + 2,
]);
{
    println!("{}", context.state()); // 1
    let context = context.resume().unwrap();
    println!("{}", context.state()); // 2
    let context = context.resume().unwrap();
    println!("{}", context.state()); // 4
}
// IS EQUIVALENT TO:
{
    println!("{}", context.consume()); // 4
}
```

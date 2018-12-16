# Deferred
Rust crate to help perform deferred execution of code logic.

[![Travis CI](https://travis-ci.org/PsichiX/deferred.svg?branch=master)](https://travis-ci.org/PsichiX/deferred)
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
deferred = "1.1"
```

Your crate module:
```rust
#[macro_use]
extern crate deferred;

use deferred::*;

fn foo(v: i32) -> Deferred<i32> {
    deferred!(v, [
        |c| state!(c.state() + 1),
        |c| foo2(c.state()).into(),
        |c| state!(c.state() + 2)
    ])
}

fn foo2(v: i32) -> Deferred<i32> {
    deferred!(v, [
      |c| state!(c.state() * 2),
      |c| state!(c.state() * 3)
    ])
}

{
  let d = foo(1);
  assert!(d.can_resume());
  assert_eq!(d.state(), Some(&1));

  let d = d.resume().unwrap();
  assert!(d.can_resume());
  assert_eq!(d.state(), Some(&2));

  let d = d.resume().unwrap();
  assert!(d.can_resume());
  assert_eq!(d.state(), Some(&4));

  let d = d.resume().unwrap();
  assert!(d.can_resume());
  assert_eq!(d.state(), Some(&12));

  let d = d.resume().unwrap();
  assert!(!d.can_resume());
  assert_eq!(d.state(), Some(&14));
}
// IS EQUIVALENT TO:
{
  let d = foo(1);
  assert_eq!(d.consume(), 14);
}
```

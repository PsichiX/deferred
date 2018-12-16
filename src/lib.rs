//! Rust crate to help perform deferred execution of code logic.
//!
//! # Problems that `deferred` crate helps to solve
//! Probably at some point in your project you will want to make a function that can have
//! partitioned logic and you want to call each of that parts at some strictly defined time
//! specified by you.
//! ```
//! # #[macro_use] extern crate deferred;
//! # use deferred::*;
//! # fn main() {
//! fn foo(v: i32) -> Deferred<i32> {
//!     deferred!(v, [
//!         |c| state!(c.state() + 1),
//!         |c| foo2(c.state()).into(),
//!         |c| state!(c.state() + 2)
//!     ])
//! }
//!
//! fn foo2(v: i32) -> Deferred<i32> {
//!     deferred!(v, [
//!         |c| state!(c.state() * 2),
//!         |c| state!(c.state() * 3)
//!     ])
//! }
//!
//! let d = foo(1);
//! assert_eq!(d.state(), Some(&1));
//! let d = d.resume().unwrap();
//! assert_eq!(d.state(), Some(&2));
//! let d = d.resume().unwrap();
//! assert_eq!(d.state(), Some(&4));
//! let d = d.resume().unwrap();
//! assert_eq!(d.state(), Some(&12));
//! let d = d.resume().unwrap();
//! assert_eq!(d.state(), Some(&14));
//! assert_eq!(d.can_resume(), false);
//! # }
//! ```
//!
//! You can think of it as staticaly defined `Promise`-like abstraction known in JavaScript or
//! other languages with high abstraction of deferred code execution.
//!
//! # It is not based on threads
//! Main reason that this crate was created is that when you work with WASM target, you cannot use
//! `Futures` or threads but you still need to run some of your code asynchronously, most likely
//! execute heavy/long calculations "in background" and you cannot make browser freeze.
//!
//! # Need to use undefined state type? Look, there is `Value` wrapper!
//! Sometimes you cannot have the same context input and output types, for example:
//! ```ignore
//! fn foo(v: i32) -> Deferred<String> {
//!     deferred!(v, [
//!         |c| state!(c.state() + 1),
//!         |c| state!(format!("{}", c.state()))
//!     ])
//! }
//!
//! let result: String = foo(42).consume();
//! ```
//! Code above gets `i32` as input and expects that at the end we get `String` value and it does
//! not compile. You could solve it by making tuple with options of each types used in context
//! inputs and outputs, like this:
//! ```
//! # #[macro_use] extern crate deferred;
//! # use deferred::*;
//! # fn main() {
//! type State = (Option<i32>, Option<String>);
//!
//! fn foo(v: i32) -> Deferred<State> {
//!     deferred!((Some(v), None), [
//!         |c| state!((Some(c.state().0.unwrap() + 1), None)),
//!         |c| state!((None, Some(format!("{}", c.state().0.unwrap()))))
//!     ])
//! }
//!
//! let result = foo(41).consume().1.unwrap();
//! assert_eq!(&result, "42");
//! # }
//! ```
//! but this looks ugly and gets even worse when you have much much more types to use - we do not
//! want that. We can use `Value` type which is basically a boxed wrapper of any value (that means:
//! you have to deal with a little runtime overhead because of storing and accessing value on heap).
//!
//! Here is how to use `Value` as state:
//! ```
//! # #[macro_use] extern crate deferred;
//! # use deferred::*;
//! # fn main() {
//! fn foo(v: i32) -> Deferred<Value> {
//!     deferred!(value!(v), [
//!         |c| state!(value!(c.state().consume::<i32>() + 1)),
//!         |c| state!(value!(format!("{}", c.state().consume::<i32>())))
//!     ])
//! }
//!
//! let result = foo(41).consume().consume::<String>();
//! assert_eq!(&result, "42");
//! # }
//! ```

pub mod context;
pub mod deferred;
pub mod deferred_manager;
mod macros;
mod tests;
pub mod value;

pub use crate::context::*;
pub use crate::deferred::*;
pub use crate::deferred_manager::*;
pub use crate::value::*;

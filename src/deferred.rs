use crate::context::*;
use std::collections::VecDeque;

/// Alias for deferred logic part that takes current context and produces new one that will be
/// passed to next deferred step execution.
pub type Part<S> = fn(input: Context<S>) -> Context<S>;

/// Struct that holds parts and state of deferred logic to execute whenever you want to.
///
/// # Note
/// Everytime when you want to resume execution, you consume deferred context and produce new one
/// so keep in mind to restore it before `resume()` and store it again after `resume()`.
pub struct Deferred<S> {
    parts: VecDeque<Part<S>>,
    context: Context<S>,
}

impl<S> Deferred<S> {
    /// Creates new deferred execution.
    ///
    /// # Arguments
    /// * `state` - context initial state.
    /// * `parts` - vector of logic parts.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// fn foo(v: i32) -> Deferred<i32> {
    ///     deferred!(v, [
    ///         |c| state!(c.state() + 1),
    ///         |c| state!(c.state() + 2)
    ///     ])
    /// }
    ///
    /// assert_eq!(foo(1).consume(), 4);
    /// # }
    /// ```
    pub fn new(state: S, parts: Vec<Part<S>>) -> Self {
        let mut p = VecDeque::new();
        p.extend(parts);
        Self {
            parts: p,
            context: Context::State(state),
        }
    }

    /// Tells if deferred execution can be resumed.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// fn foo(v: i32) -> Deferred<i32> {
    ///     deferred!(v, [
    ///         |c| state!(c.state() + 1),
    ///         |c| state!(c.state() + 2)
    ///     ])
    /// }
    ///
    /// let d = foo(1);
    /// assert_eq!(d.can_resume(), true);
    /// let d = d.resume().unwrap();
    /// assert_eq!(d.can_resume(), true);
    /// let d = d.resume().unwrap();
    /// assert_eq!(d.can_resume(), false);
    /// # }
    /// ```
    pub fn can_resume(&self) -> bool {
        match &self.context {
            Context::State(_) => !self.parts.is_empty(),
            Context::Deferred(d) => d.can_resume() || !self.parts.is_empty(),
        }
    }

    /// Gets reference to current state stored in context.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// fn foo(v: i32) -> Deferred<i32> {
    ///     deferred!(v, [
    ///         |c| state!(c.state() + 1),
    ///         |c| state!(c.state() + 2)
    ///     ])
    /// }
    ///
    /// let d = foo(1);
    /// assert_eq!(d.state(), Some(&1));
    /// let d = d.resume().unwrap();
    /// assert_eq!(d.state(), Some(&2));
    /// let d = d.resume().unwrap();
    /// assert_eq!(d.state(), Some(&4));
    /// # }
    /// ```
    pub fn state(&self) -> Option<&S> {
        self.context.get_state()
    }

    /// Resumes deferred execution, which means we execute next logic part and store its state.
    ///
    /// # Note
    /// While you resume execution, you consume it and return new one so keep in mind that you need
    /// to store it again or replace with old one after calling `resume()`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// fn foo(v: i32) -> Deferred<i32> {
    ///     deferred!(v, [
    ///         |c| state!(c.state() + 1),
    ///         |c| foo2(c.state()).into(),
    ///         |c| state!(c.state() + 2)
    ///     ])
    /// }
    ///
    /// fn foo2(v: i32) -> Deferred<i32> {
    ///     deferred!(v, [
    ///         |c| state!(c.state() * 2),
    ///         |c| state!(c.state() * 3)
    ///     ])
    /// }
    ///
    /// let d = foo(1);
    /// assert!(d.can_resume());
    /// assert_eq!(d.state(), Some(&1));
    ///
    /// let d = d.resume().unwrap();
    /// assert!(d.can_resume());
    /// assert_eq!(d.state(), Some(&2));
    ///
    /// let d = d.resume().unwrap();
    /// assert!(d.can_resume());
    /// assert_eq!(d.state(), Some(&4));
    ///
    /// let d = d.resume().unwrap();
    /// assert!(d.can_resume());
    /// assert_eq!(d.state(), Some(&12));
    ///
    /// let d = d.resume().unwrap();
    /// assert!(!d.can_resume());
    /// assert_eq!(d.state(), Some(&14));
    /// # }
    /// ```
    pub fn resume(mut self) -> Option<Self> {
        match self.context {
            Context::State(state) => {
                if let Some(part) = self.parts.pop_front() {
                    let context = part(Context::State(state));
                    if context.is_deferred() {
                        self.context = context;
                        self.resume()
                    } else {
                        self.context = context;
                        Some(self)
                    }
                } else {
                    None
                }
            }
            Context::Deferred(deferred) => {
                if deferred.can_resume() {
                    if let Some(deferred) = deferred.resume() {
                        self.context = deferred.into();
                        Some(self)
                    } else {
                        None
                    }
                } else {
                    self.context = Context::State(deferred.consume());
                    self.resume()
                }
            }
        }
    }

    /// Consumes deferred execution, which means we execute all remaining logic parts and returns
    /// final state.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// fn foo(v: i32) -> Deferred<i32> {
    ///     deferred!(v, [
    ///         |c| state!(c.state() + 1),
    ///         |c| state!(c.state() + 2)
    ///     ])
    /// }
    ///
    /// assert_eq!(foo(1).consume(), 4);
    /// # }
    /// ```
    pub fn consume(mut self) -> S {
        while self.can_resume() {
            self = self.resume().unwrap();
        }
        self.context.state()
    }

    /// Alias for `consume()` method.
    #[inline]
    pub fn unwrap(self) -> S {
        self.consume()
    }
}

impl<S> Into<Context<S>> for Deferred<S> {
    fn into(self) -> Context<S> {
        Context::Deferred(Box::new(self))
    }
}

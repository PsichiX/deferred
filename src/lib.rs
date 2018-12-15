use std::collections::VecDeque;

/// Alias for deferred logic part that takes current state and produces next state.
pub type DeferredPart<S> = fn(S) -> S;

/// Struct that holds parts and state of deferred logic to execute whenever you want to.
///
/// NOTE: everytime when you want to resume execution, you consume deferred context and procuce new
/// one so keep in mind to restore it before [resume] and store again after [resume].
pub struct Deferred<S>
where
    S: Send + Sync,
{
    parts: VecDeque<DeferredPart<S>>,
    state: S,
}

impl<S> Deferred<S>
where
    S: Send + Sync,
{
    /// Creates new deferred execution context.
    ///
    /// # Arguments
    /// * `state` - context initial state.
    /// * `parts` - vector of logic parts.
    ///
    /// # Example
    /// ```
    /// use deferred::Deferred;
    ///
    /// let context = Deferred::new(1, vec![
    ///     |v| v + 1,
    ///     |v| v + 2,
    /// ]);
    /// let context = context.resume().unwrap();
    /// println!("{}", context.state()); // 2
    /// let context = context.resume().unwrap();
    /// println!("{}", context.state()); // 4
    /// ```
    pub fn new(state: S, parts: Vec<DeferredPart<S>>) -> Self {
        let mut p = VecDeque::new();
        p.extend(parts);
        Self { parts: p, state }
    }

    /// Gets number of steps needed to complete execution.
    ///
    /// # Example
    /// ```
    /// use deferred::Deferred;
    ///
    /// let context = Deferred::new(1, vec![
    ///     |v| v + 1,
    ///     |v| v + 2,
    /// ]);
    /// assert_eq!(context.steps_left(), 2);
    /// let context = context.resume().unwrap();
    /// assert_eq!(context.steps_left(), 1);
    /// let context = context.resume().unwrap();
    /// assert_eq!(context.steps_left(), 0);
    /// ```
    pub fn steps_left(&self) -> usize {
        self.parts.len()
    }

    /// Tells if deferred execution context can be resumed.
    ///
    /// # Example
    /// ```
    /// use deferred::Deferred;
    ///
    /// let context = Deferred::new(1, vec![
    ///     |v| v + 1,
    ///     |v| v + 2,
    /// ]);
    /// assert!(context.can_resume());
    /// let context = context.resume().unwrap();
    /// assert!(context.can_resume());
    /// let context = context.resume().unwrap();
    /// assert!(!context.can_resume());
    /// ```
    pub fn can_resume(&self) -> bool {
        !self.parts.is_empty()
    }

    /// Gets reference to current state stored in context.
    ///
    /// # Example
    /// ```
    /// use deferred::Deferred;
    ///
    /// let context = Deferred::new(1, vec![
    ///     |v| v + 1,
    ///     |v| v + 2,
    /// ]);
    /// assert_eq!(context.state(), &1);
    /// let context = context.resume().unwrap();
    /// assert_eq!(context.state(), &2);
    /// let context = context.resume().unwrap();
    /// assert_eq!(context.state(), &4);
    /// ```
    pub fn state(&self) -> &S {
        &self.state
    }

    /// Resumes deferred execution context, which means we execute next logic part and store its
    /// state.
    /// NOTE: While you resume context, you consume it and return new one.
    ///
    /// # Example
    /// ```
    /// use deferred::Deferred;
    ///
    /// let context = Deferred::new(1, vec![
    ///     |v| v + 1,
    ///     |v| v + 2,
    /// ]);
    /// assert_eq!(context.state(), &1);
    /// let context = context.resume().unwrap();
    /// assert_eq!(context.state(), &2);
    /// let context = context.resume().unwrap();
    /// assert_eq!(context.state(), &4);
    /// ```
    pub fn resume(mut self) -> Option<Self> {
        if let Some(part) = self.parts.pop_front() {
            let state = self.state;
            self.state = part(state);
            Some(self)
        } else {
            None
        }
    }

    /// Consumes deferred execution context, which means we execute all remaining logic parts and
    /// returns state.
    ///
    /// # Example
    /// ```
    /// use deferred::Deferred;
    ///
    /// let context = Deferred::new(1, vec![
    ///     |v| v + 1,
    ///     |v| v + 2,
    /// ]);
    /// assert_eq!(context.consume(), 4);
    /// ```
    pub fn consume(mut self) -> S {
        while self.can_resume() {
            self = self.resume().unwrap();
        }
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resume() {
        fn foo(v: i32) -> Deferred<i32> {
            Deferred::new(
                v,
                vec![
                    |v| {
                        println!("= part 1");
                        v + 1
                    },
                    |v| {
                        println!("= part 2");
                        v + 2
                    },
                ],
            )
        }

        let d = foo(1);
        assert_eq!(d.steps_left(), 2);
        assert!(d.can_resume());
        assert_eq!(d.state(), &1);

        let d = d.resume().unwrap();
        assert_eq!(d.steps_left(), 1);
        assert!(d.can_resume());
        assert_eq!(d.state(), &2);

        let d = d.resume().unwrap();
        assert_eq!(d.steps_left(), 0);
        assert!(!d.can_resume());
        assert_eq!(d.state(), &4);
    }

    #[test]
    fn test_consume() {
        fn foo(v: i32) -> Deferred<i32> {
            Deferred::new(
                v,
                vec![
                    |v| {
                        println!("= part 1");
                        v + 1
                    },
                    |v| {
                        println!("= part 2");
                        v + 2
                    },
                ],
            )
        }

        assert_eq!(foo(1).consume(), 4);
    }
}

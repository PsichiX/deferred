use crate::deferred::*;

/// Deferred execution context holds its state or inner deferred execution (if there is deferred
/// subroutine needed to evaluate).
pub enum Context<S> {
    /// Context holds single state.
    State(S),
    /// Context holds deferred subroutine needed to evaluate.
    Deferred(Box<Deferred<S>>),
}

impl<S> Context<S> {
    /// Tells if context holds a state.
    pub fn is_state(&self) -> bool {
        if let Context::State(_) = self {
            true
        } else {
            false
        }
    }

    /// Tells if context holds a deferred subroutine to evaluate.
    pub fn is_deferred(&self) -> bool {
        if let Context::Deferred(_) = self {
            true
        } else {
            false
        }
    }

    /// Gets reference to current state if there is one hold by context or its deferred subroutine.
    pub fn get_state(&self) -> Option<&S> {
        match self {
            Context::State(state) => Some(state),
            Context::Deferred(deferred) => deferred.state(),
        }
    }

    /// Gets deferred subroutine if context has one.
    pub fn get_deferred(&self) -> Option<&Deferred<S>> {
        if let Context::Deferred(deferred) = self {
            Some(deferred)
        } else {
            None
        }
    }

    /// Consumes context and returns its state.
    pub fn state(self) -> S {
        match self {
            Context::State(state) => state,
            Context::Deferred(deferred) => deferred.consume(),
        }
    }

    /// Consumes context and returns its deferred subroutine.
    ///
    /// # Panics
    /// * when context does not hold deferred subroutine so you should make sure about that by
    ///   calling `self.is_deferred()` before gettin context deferred subroutine.
    pub fn deferred(self) -> Deferred<S> {
        if let Context::Deferred(deferred) = self {
            *deferred
        } else {
            panic!("Trying to get deferred execution of context that does not have a deferred execution")
        }
    }

    /// Alias for `state()` method.
    #[inline]
    pub fn unwrap(self) -> S {
        self.state()
    }
}

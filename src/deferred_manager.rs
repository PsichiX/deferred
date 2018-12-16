use crate::deferred::*;
use std::collections::HashMap;

/// Alias for deferred execution identifier;
pub type Id = usize;

/// Deferred execution manager used to store and resume.
pub struct DeferredManager<S> {
    registry: HashMap<Id, Deferred<S>>,
    id_generator: Id,
}

impl<S> DeferredManager<S> {
    /// Creates new deferred execution manager.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// use std::rc::Rc;
    /// use std::cell::Cell;
    ///
    /// type RcBool = Rc<Cell<bool>>;
    ///
    /// fn foo(v: RcBool) -> Deferred<Value> {
    ///     deferred!(value!(v), [
    ///         |c| {
    ///             let v = c.state().consume::<RcBool>();
    ///             v.set(true);
    ///             state!(value!(v))
    ///         }
    ///     ])
    /// }
    ///
    /// let mut manager = DeferredManager::new();
    /// let status = Rc::new(Cell::new(false));
    /// let id = manager.run(foo(status.clone()));
    /// assert_eq!(manager.has(id), true);
    /// assert_eq!(status.get(), false);
    /// manager.resume_all();
    /// assert_eq!(manager.has(id), false);
    /// assert_eq!(status.get(), true);
    /// # }
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets number of deferred executions currently waiting to resume.
    #[inline]
    pub fn count(&self) -> usize {
        self.registry.len()
    }

    /// Register deferred logic for later execution.
    ///
    /// # Arguments
    /// * `deferred` - deferred execution unit.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// use std::rc::Rc;
    /// use std::cell::Cell;
    ///
    /// type RcBool = Rc<Cell<bool>>;
    ///
    /// fn foo(v: RcBool) -> Deferred<Value> {
    ///     deferred!(value!(v), [
    ///         |c| {
    ///             let v = c.state().consume::<RcBool>();
    ///             v.set(true);
    ///             state!(value!(v))
    ///         }
    ///     ])
    /// }
    ///
    /// let mut manager = DeferredManager::new();
    /// let status = Rc::new(Cell::new(false));
    /// let id = manager.run(foo(status.clone()));
    /// assert_eq!(manager.has(id), true);
    /// assert_eq!(status.get(), false);
    /// manager.resume_all();
    /// assert_eq!(manager.has(id), false);
    /// assert_eq!(status.get(), true);
    /// # }
    /// ```
    pub fn run(&mut self, deferred: Deferred<S>) -> Id {
        let id = self.id_generator;
        self.id_generator += 1;
        self.registry.insert(id, deferred);
        id
    }

    /// Creates new deferred execution manager.
    ///
    /// # Arguments
    /// * `id` - deferred execution id (got from calling `run()` method).
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// use std::rc::Rc;
    /// use std::cell::Cell;
    ///
    /// type RcBool = Rc<Cell<bool>>;
    ///
    /// fn foo(v: RcBool) -> Deferred<Value> {
    ///     deferred!(value!(v), [
    ///         |c| {
    ///             let v = c.state().consume::<RcBool>();
    ///             v.set(true);
    ///             state!(value!(v))
    ///         }
    ///     ])
    /// }
    ///
    /// let mut manager = DeferredManager::new();
    /// let status = Rc::new(Cell::new(false));
    /// let id = manager.run(foo(status.clone()));
    /// assert_eq!(manager.has(id), true);
    /// assert_eq!(status.get(), false);
    /// manager.cancel(id);
    /// assert_eq!(manager.has(id), false);
    /// assert_eq!(status.get(), false);
    /// # }
    /// ```
    #[inline]
    pub fn cancel(&mut self, id: Id) -> bool {
        self.registry.remove(&id).is_some()
    }

    /// Resume specified deferred execution unit by its id.
    ///
    /// # Arguments
    /// * `id` - deferred execution id (got from calling `run()` method).
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// use std::rc::Rc;
    /// use std::cell::Cell;
    ///
    /// type RcBool = Rc<Cell<bool>>;
    ///
    /// fn foo(v: RcBool) -> Deferred<Value> {
    ///     deferred!(value!(v), [
    ///         |c| {
    ///             let v = c.state().consume::<RcBool>();
    ///             v.set(true);
    ///             state!(value!(v))
    ///         }
    ///     ])
    /// }
    ///
    /// let mut manager = DeferredManager::new();
    /// let status = Rc::new(Cell::new(false));
    /// let id = manager.run(foo(status.clone()));
    /// assert_eq!(manager.has(id), true);
    /// assert_eq!(status.get(), false);
    /// manager.resume(id);
    /// assert_eq!(manager.has(id), false);
    /// assert_eq!(status.get(), true);
    /// # }
    /// ```
    #[inline]
    pub fn resume(&mut self, id: Id) -> bool {
        if let Some(deferred) = self.registry.remove(&id) {
            if let Some(deferred) = deferred.resume() {
                if deferred.can_resume() {
                    self.registry.insert(id, deferred);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Consume specified deferred execution unit by its id and return its state.
    ///
    /// # Arguments
    /// * `id` - deferred execution id (got from calling `run()` method).
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// use std::rc::Rc;
    /// use std::cell::Cell;
    ///
    /// type RcBool = Rc<Cell<bool>>;
    ///
    /// fn foo(v: RcBool) -> Deferred<Value> {
    ///     deferred!(value!(v), [
    ///         |c| {
    ///             let v = c.state().consume::<RcBool>();
    ///             v.set(true);
    ///             state!(value!(v))
    ///         }
    ///     ])
    /// }
    ///
    /// let mut manager = DeferredManager::new();
    /// let status = Rc::new(Cell::new(false));
    /// let id = manager.run(foo(status.clone()));
    /// assert_eq!(manager.has(id), true);
    /// assert_eq!(status.get(), false);
    /// manager.consume(id).unwrap();
    /// assert_eq!(manager.has(id), false);
    /// assert_eq!(status.get(), true);
    /// # }
    /// ```
    #[inline]
    pub fn consume(&mut self, id: Id) -> Option<S> {
        if let Some(deferred) = self.registry.remove(&id) {
            Some(deferred.consume())
        } else {
            None
        }
    }

    /// Tells if deferred execution unit with given id currently waits for later execution.
    ///
    /// # Arguments
    /// * `id` - deferred execution id (got from calling `run()` method).
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// use std::rc::Rc;
    /// use std::cell::Cell;
    ///
    /// type RcBool = Rc<Cell<bool>>;
    ///
    /// fn foo(v: RcBool) -> Deferred<Value> {
    ///     deferred!(value!(v), [
    ///         |c| {
    ///             let v = c.state().consume::<RcBool>();
    ///             v.set(true);
    ///             state!(value!(v))
    ///         }
    ///     ])
    /// }
    ///
    /// let mut manager = DeferredManager::new();
    /// let status = Rc::new(Cell::new(false));
    /// let id = manager.run(foo(status.clone()));
    /// assert_eq!(manager.has(id), true);
    /// manager.resume_all();
    /// assert_eq!(manager.has(id), false);
    /// # }
    /// ```
    #[inline]
    pub fn has(&self, id: Id) -> bool {
        self.registry.contains_key(&id)
    }

    /// Resume sall deferred execution units.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// use std::rc::Rc;
    /// use std::cell::Cell;
    ///
    /// type RcBool = Rc<Cell<bool>>;
    ///
    /// fn foo(v: RcBool) -> Deferred<Value> {
    ///     deferred!(value!(v), [
    ///         |c| {
    ///             let v = c.state().consume::<RcBool>();
    ///             v.set(true);
    ///             state!(value!(v))
    ///         }
    ///     ])
    /// }
    ///
    /// let mut manager = DeferredManager::new();
    /// let status = Rc::new(Cell::new(false));
    /// let id = manager.run(foo(status.clone()));
    /// assert_eq!(manager.has(id), true);
    /// assert_eq!(status.get(), false);
    /// manager.resume_all();
    /// assert_eq!(manager.has(id), false);
    /// assert_eq!(status.get(), true);
    /// # }
    /// ```
    pub fn resume_all(&mut self) {
        let mut registry = HashMap::new();
        let kv = self.registry.drain().filter_map(|(i, d)| {
            if let Some(d) = d.resume() {
                if d.can_resume() {
                    Some((i, d))
                } else {
                    None
                }
            } else {
                None
            }
        });
        for (i, d) in kv {
            registry.insert(i, d);
        }
        self.registry = registry;
    }

    /// Consume all deferred execution units and return vector of id-state pairs.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate deferred;
    /// # use deferred::*;
    /// # fn main() {
    /// use std::rc::Rc;
    /// use std::cell::Cell;
    ///
    /// type RcBool = Rc<Cell<bool>>;
    ///
    /// fn foo(v: RcBool) -> Deferred<Value> {
    ///     deferred!(value!(v), [
    ///         |c| {
    ///             let v = c.state().consume::<RcBool>();
    ///             v.set(true);
    ///             state!(value!(v))
    ///         }
    ///     ])
    /// }
    ///
    /// let mut manager = DeferredManager::new();
    /// let status = Rc::new(Cell::new(false));
    /// let id = manager.run(foo(status.clone()));
    /// assert_eq!(manager.has(id), true);
    /// assert_eq!(status.get(), false);
    /// manager.consume_all();
    /// assert_eq!(manager.has(id), false);
    /// assert_eq!(status.get(), true);
    /// # }
    /// ```
    pub fn consume_all(&mut self) -> Vec<(Id, S)> {
        self.registry
            .drain()
            .filter_map(|(i, d)| {
                if d.can_resume() {
                    Some((i, d.consume()))
                } else {
                    None
                }
            })
            .collect::<Vec<(Id, S)>>()
    }
}

impl<S> Default for DeferredManager<S> {
    fn default() -> Self {
        Self {
            registry: HashMap::new(),
            id_generator: 0,
        }
    }
}

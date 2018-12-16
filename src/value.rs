use std::any::Any;

/// Wrapper over value of non-specified type.
///
/// # Example
/// ```
/// # #[macro_use] extern crate deferred;
/// # use deferred::Value;
/// # fn main() {
/// let v = Value::new(Box::new(42));
/// assert_eq!(v.is::<i32>(), true);
/// assert_eq!(v.is::<f32>(), false);
/// assert_eq!(v.get::<i32>(), Some(&42));
/// assert_eq!(v.into_cloned::<i32>(), Some(42));
/// assert_eq!(v.into_cloned::<f32>(), None);
/// assert_eq!(v.unwrap::<i32>(), &42);
/// assert_eq!(v.consume::<i32>(), 42);
/// # }
/// ```
pub struct Value {
    inner: Box<Any>,
}

impl Value {
    /// Returns new value.
    ///
    /// # Arguments
    /// * `value` - boxed value of any type.
    pub fn new(value: Box<Any>) -> Self {
        Self { inner: value }
    }

    /// Tells if value is type of given type.
    #[inline]
    pub fn is<T: 'static>(&self) -> bool {
        self.inner.is::<T>()
    }

    /// Gets reference to value of given type or `None` if its not of that type.
    #[inline]
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.inner.downcast_ref::<T>()
    }

    /// Gets mutable reference to value of given type or `None` if its not of that type.
    #[inline]
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.inner.downcast_mut::<T>()
    }

    /// Gets cloned value of given type or `None` if its not of that type.
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub fn into_cloned<T: 'static>(&self) -> Option<T>
    where
        T: Clone,
    {
        self.get::<T>().cloned()
    }

    /// Gets reference to value of given type or panics if its not of that type.
    ///
    /// # Panics
    /// * when trying to use target type other than that of inner value.
    #[inline]
    pub fn unwrap<T: 'static>(&self) -> &T {
        self.get::<T>().unwrap()
    }

    /// Gets mutable reference to value of given type or panics if its not of that type.
    ///
    /// # Panics
    /// * when trying to use target type other than that of inner value.
    #[inline]
    pub fn unwrap_mut<T: 'static>(&mut self) -> &mut T {
        self.get_mut::<T>().unwrap()
    }

    /// Consumes value of given type and returns it or panics if its not of that type.
    ///
    /// # Panics
    /// * when trying to use target type other than that of inner value.
    #[inline]
    pub fn consume<T: 'static>(self) -> T
    where
        T: Clone,
    {
        self.into_cloned::<T>().unwrap()
    }
}

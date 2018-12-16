#[macro_export]
macro_rules! deferred {
    ( $s:expr, [$($v:expr),*] ) => {
        crate::Deferred::new($s, vec![$($v,)*])
    };
    ( $s:expr ) => {
        crate::Deferred::new($s, vec![])
    };
}

#[macro_export]
macro_rules! state {
    ( $s:expr ) => {
        crate::Context::State($s)
    };
}

#[macro_export]
macro_rules! subdeferred {
    ( $s:expr, [$($v:expr),*] ) => {
        crate::Context::Deferred(Box::new(crate::Deferred::new($s, vec![$($v,)*])))
    };
    ( $s:expr ) => {
        crate::Context::Deferred(Box::new(crate::Deferred::new($s, vec![])))
    };
}

#[macro_export]
macro_rules! value {
    ( $v:expr ) => {
        crate::value::Value::new(Box::new($v))
    };
}

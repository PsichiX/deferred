#![cfg(test)]
use crate::*;

#[test]
fn test_resume_consume() {
    fn foo(v: i32) -> Deferred<i32> {
        deferred!(v, [|c| state!(c.state() + 1), |c| state!(c.state() + 2)])
    }

    {
        let d = foo(1);
        assert!(d.can_resume());
        assert_eq!(d.state(), Some(&1));

        let d = d.resume().unwrap();
        assert!(d.can_resume());
        assert_eq!(d.state(), Some(&2));

        let d = d.resume().unwrap();
        assert!(!d.can_resume());
        assert_eq!(d.state(), Some(&4));
    }
    {
        let d = foo(1);
        assert_eq!(d.consume(), 4);
    }
}

#[test]
fn test_nested() {
    fn foo(v: i32) -> Deferred<i32> {
        deferred!(
            v,
            [
                |c| state!(c.state() + 1),
                |c| foo2(c.state()).into(),
                |c| state!(c.state() + 2)
            ]
        )
    }

    fn foo2(v: i32) -> Deferred<i32> {
        deferred!(v, [|c| state!(c.state() * 2), |c| state!(c.state() * 3)])
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
    {
        let d = foo(1);
        assert_eq!(d.consume(), 14);
    }
}

#[test]
fn test_value() {
    use crate::value::*;

    fn foo(v: i32) -> Deferred<Value> {
        deferred!(
            value!(v),
            [
                |c| state!(value!(c.state().consume::<i32>() + 1)),
                |c| {
                    let v = c.state().consume::<i32>();
                    let s = format!("Incremented value: {}", v);
                    state!(value!(s))
                }
            ]
        )
    }

    {
        let d = foo(1);
        assert_eq!(*d.state().unwrap().unwrap::<i32>(), 1);
        let d = d.resume().unwrap();
        assert_eq!(*d.state().unwrap().unwrap::<i32>(), 2);
        let d = d.resume().unwrap();
        assert_eq!(
            d.state().unwrap().unwrap::<String>(),
            "Incremented value: 2"
        );
    }
    {
        let d = foo(1);
        assert_eq!(d.consume().unwrap::<String>(), "Incremented value: 2");
    }
}

#[test]
fn test_manager() {
    use std::cell::Cell;
    use std::rc::Rc;

    type RcBool = Rc<Cell<bool>>;

    fn foo(v: RcBool) -> Deferred<RcBool> {
        deferred!(
            v,
            [
                |c| c,
                |c| {
                    let v = c.state();
                    v.set(true);
                    state!(v)
                }
            ]
        )
    }

    fn foo2(v: RcBool) -> Deferred<RcBool> {
        deferred!(
            v,
            [|c| {
                let v = c.state();
                v.set(true);
                state!(v)
            }]
        )
    }

    let mut manager = DeferredManager::new();
    {
        let status = Rc::new(Cell::new(false));
        let status2 = Rc::new(Cell::new(false));
        assert_eq!(manager.count(), 0);

        let id = manager.run(foo(status.clone()));
        let id2 = manager.run(foo2(status2.clone()));
        assert_eq!(manager.count(), 2);
        assert_eq!(manager.has(id), true);
        assert_eq!(manager.has(id2), true);
        assert_eq!(status.get(), false);
        assert_eq!(status2.get(), false);

        manager.resume_all();
        assert_eq!(manager.count(), 1);
        assert_eq!(manager.has(id), true);
        assert_eq!(manager.has(id2), false);
        assert_eq!(status.get(), false);
        assert_eq!(status2.get(), true);

        manager.resume_all();
        assert_eq!(manager.count(), 0);
        assert_eq!(manager.has(id), false);
        assert_eq!(status.get(), true);
    }
    {
        let status = Rc::new(Cell::new(false));
        let status2 = Rc::new(Cell::new(false));
        assert_eq!(manager.count(), 0);

        let id = manager.run(foo(status.clone()));
        let id2 = manager.run(foo2(status2.clone()));
        assert_eq!(manager.count(), 2);
        assert_eq!(manager.has(id), true);
        assert_eq!(manager.has(id2), true);
        assert_eq!(status.get(), false);
        assert_eq!(status2.get(), false);

        manager.consume_all();
        assert_eq!(manager.count(), 0);
        assert_eq!(manager.has(id), false);
        assert_eq!(manager.has(id2), false);
        assert_eq!(status.get(), true);
        assert_eq!(status2.get(), true);
    }
}

#[test]
fn test_manager_value() {
    use std::cell::Cell;
    use std::rc::Rc;

    type RcBool = Rc<Cell<bool>>;

    fn foo(v: RcBool) -> Deferred<Value> {
        deferred!(
            value!(v),
            [
                |c| c,
                |c| {
                    let v = c.state().consume::<RcBool>();
                    v.set(true);
                    state!(value!(v))
                }
            ]
        )
    }

    fn foo2(v: RcBool) -> Deferred<Value> {
        deferred!(
            value!(v),
            [|c| {
                let v = c.state().consume::<RcBool>();
                v.set(true);
                state!(value!(v))
            }]
        )
    }

    let mut manager = DeferredManager::new();
    {
        let status = Rc::new(Cell::new(false));
        let status2 = Rc::new(Cell::new(false));
        assert_eq!(manager.count(), 0);

        let id = manager.run(foo(status.clone()));
        let id2 = manager.run(foo2(status2.clone()));
        assert_eq!(manager.count(), 2);
        assert_eq!(manager.has(id), true);
        assert_eq!(manager.has(id2), true);
        assert_eq!(status.get(), false);
        assert_eq!(status2.get(), false);

        manager.resume_all();
        assert_eq!(manager.count(), 1);
        assert_eq!(manager.has(id), true);
        assert_eq!(manager.has(id2), false);
        assert_eq!(status.get(), false);
        assert_eq!(status2.get(), true);

        manager.resume_all();
        assert_eq!(manager.count(), 0);
        assert_eq!(manager.has(id), false);
        assert_eq!(status.get(), true);
    }
    {
        let status = Rc::new(Cell::new(false));
        let status2 = Rc::new(Cell::new(false));
        assert_eq!(manager.count(), 0);

        let id = manager.run(foo(status.clone()));
        let id2 = manager.run(foo2(status2.clone()));
        assert_eq!(manager.count(), 2);
        assert_eq!(manager.has(id), true);
        assert_eq!(manager.has(id2), true);
        assert_eq!(status.get(), false);
        assert_eq!(status2.get(), false);

        manager.consume_all();
        assert_eq!(manager.count(), 0);
        assert_eq!(manager.has(id), false);
        assert_eq!(manager.has(id2), false);
        assert_eq!(status.get(), true);
        assert_eq!(status2.get(), true);
    }
}

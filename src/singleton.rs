pub trait Singleton {
    type Container;

    fn singleton() -> Self::Container;
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_singleton_base {
    ($ty:ty,$ctnr:ty,$init:expr) => {
        impl Singleton for $ty {
            type Container = $ctnr;

            fn singleton() -> Self::Container {
                static mut INSTANCE: Option<$ctnr> = None;
                static START: std::sync::Once = std::sync::Once::new();

                START.call_once(|| unsafe { INSTANCE = Some($init) });

                unsafe { INSTANCE.clone().unwrap() }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_singleton_rc {
    ($ty:ty,$init:expr) => {
        __impl_singleton_base!($ty, std::rc::Rc<$ty>, std::rc::Rc::from($init));
    };
}

#[macro_export]
macro_rules! impl_singleton_rc_cell {
    ($ty:ty,$init:expr) => {
        __impl_singleton_base!(
            $ty,
            std::rc::Rc<std::cell::Cell<$ty>>,
            std::rc::Rc::from(std::cell::Cell::from($init))
        );
    };
}

#[macro_export]
macro_rules! impl_singleton_rc_refcell {
    ($ty:ty,$init:expr) => {
        __impl_singleton_base!(
            $ty,
            std::rc::Rc<std::cell::RefCell<$ty>>,
            std::rc::Rc::from(std::cell::RefCell::from($init))
        );
    };
}

#[macro_export]
macro_rules! impl_singleton_arc {
    ($ty:ty,$init:expr) => {
        __impl_singleton_base!($ty, std::sync::Arc<$ty>, std::sync::Arc::from($init));
    };
}

#[macro_export]
macro_rules! impl_singleton_arc_mutex {
    ($ty:ty,$init:expr) => {
        __impl_singleton_base!(
            $ty,
            std::sync::Arc<std::sync::Mutex<$ty>>,
            std::sync::Arc::from(std::sync::Mutex::from($init))
        );
    };
}

#[macro_export]
macro_rules! impl_singleton_arc_rwlock {
    ($ty:ty,$init:expr) => {
        __impl_singleton_base!(
            $ty,
            std::sync::Arc<std::sync::RwLock<$ty>>,
            std::sync::Arc::from(std::sync::RwLock::from($init))
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn rc_test() {
        struct Foo;
        impl_singleton_rc!(Foo, Foo);
        assert!(Rc::ptr_eq(&Foo::singleton(), &Foo::singleton()));
    }

    #[test]
    fn rc_cell_test() {
        struct Foo;
        impl_singleton_rc_cell!(Foo, Foo);
        assert!(Rc::ptr_eq(&Foo::singleton(), &Foo::singleton()));
    }

    #[test]
    fn rc_refcell_test() {
        struct Foo;
        impl_singleton_rc_refcell!(Foo, Foo);
        assert!(Rc::ptr_eq(&Foo::singleton(), &Foo::singleton()));
    }

    #[test]
    fn arc_test() {
        struct Foo;
        impl_singleton_arc!(Foo, Foo);
        assert!(Arc::ptr_eq(&Foo::singleton(), &Foo::singleton()));
    }

    #[test]
    fn arc_mutex_test() {
        struct Foo(i32);
        impl_singleton_arc_mutex!(Foo, Foo(0));
        assert!(Arc::ptr_eq(&Foo::singleton(), &Foo::singleton()));
    }

    #[test]
    fn arc_rwlock_test() {
        struct Foo;
        impl_singleton_arc_rwlock!(Foo, Foo);
        assert!(Arc::ptr_eq(&Foo::singleton(), &Foo::singleton()));
    }
}

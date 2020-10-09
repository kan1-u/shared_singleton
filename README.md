# Shared Singleton

The `shared_singleton` trait provides singleton pattern state management with shared container.

Implement the singleton pattern with very short code are available.  
You can choose the better shared container from `Rc<T>`, `Rc<Cell<T>>`, `Rc<RefCell<T>>`, `Arc<T>`, `Arc<Mutex<T>>` and `Arc<RwLock<T>>`.

## Examples

- `Rc<T>`

```rust
use shared_singleton::*;

struct Foo(usize);
impl_singleton_rc!(Foo, Foo(1));

let x1 = Foo::singleton();
let x2 = Foo::singleton();

assert_eq!(1, Foo::singleton().0);

assert!(std::rc::Rc::ptr_eq(&x1, &x2));
```

- `Rc<Cell<T>>`

```rust
use shared_singleton::*;

#[derive(Clone, Copy)]
struct Foo(usize);
impl_singleton_rc_cell!(Foo, Foo(1));

let x1 = Foo::singleton();
let x2 = Foo::singleton();

assert_eq!(1, x1.get().0);
x2.set(Foo(2));
assert_eq!(2, x1.get().0);

assert!(std::rc::Rc::ptr_eq(&x1, &x2));
```

- `Rc<RefCell<T>>`

```rust
use shared_singleton::*;

struct Foo(usize);
impl_singleton_rc_refcell!(Foo, Foo(1));

let x1 = Foo::singleton();
let x2 = Foo::singleton();

assert_eq!(1, x1.borrow().0);
x2.borrow_mut().0 = 2;
assert_eq!(2, x1.borrow().0);

assert!(std::rc::Rc::ptr_eq(&x1, &x2));
```

- `Arc<T>`

```rust
use shared_singleton::*;

struct Foo(usize);
impl_singleton_arc!(Foo, Foo(1));

let x1 = Foo::singleton();
let x2 = Foo::singleton();

assert_eq!(1, x1.0);

assert!(std::sync::Arc::ptr_eq(&x1, &x2));
```

- `Arc<Mutex<T>>`

```rust
use shared_singleton::*;

struct Foo(usize);
impl_singleton_arc_mutex!(Foo, Foo(1));

let x1 = Foo::singleton();
let x2 = Foo::singleton();

assert!(std::sync::Arc::ptr_eq(&x1, &x2));

let t1 = std::thread::spawn(move || {
    x1.lock().unwrap().0 = 2;
    println!("{}", x1.lock().unwrap().0);
});

let t2 = std::thread::spawn(move || {
    x2.lock().unwrap().0 = 3;
    println!("{}", x2.lock().unwrap().0);
});

t1.join().unwrap();
t2.join().unwrap();
```

- `Arc<RwLock<T>>`

```rust
use shared_singleton::*;

struct Foo(usize);
impl_singleton_arc_rwlock!(Foo, Foo(1));

let x1 = Foo::singleton();
let x2 = Foo::singleton();

assert!(std::sync::Arc::ptr_eq(&x1, &x2));

let t1 = std::thread::spawn(move || {
    x1.write().unwrap().0 = 2;
    println!("{}", x1.read().unwrap().0);
});

let t2 = std::thread::spawn(move || {
    x2.write().unwrap().0 = 3;
    println!("{}", x2.read().unwrap().0);
});

t1.join().unwrap();
t2.join().unwrap();
```

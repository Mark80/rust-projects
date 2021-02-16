use std::ops::Deref;
use std::rc::Rc;

use crate::List::{Cons, Nil};

struct Misc {
    _1: u64,
    _2: u64,
    _3: u64,
    _4: u64,
    _5: u64,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let t = &String::from("ciccio"); //borrowing

    {
        let q = t;
        println!("q {}", q);
    }

    let c = t;

    println!("t {}", t);
    println!("c {}", c);

    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and
    // stores the borrow and closure in the `print` variable. It will remain
    // borrowed until `print` is used the last time.
    //
    // `println!` only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    println!("{}", color);
    println!("{}", color);

    let double = |x| 2 * x;
    let rr = apply(double);
    let rr2 = apply_2(double);
    println!("{}", rr);
    println!("{}", rr2);
}

fn answer() -> &'static i32 {
    let x = 6;
    &x
}

fn apply<F>(f: F) -> i32 where
    F: Fn(i32) -> i32
{
    f(5)
}

fn apply_2<F: Fn(i32) -> i32>(f: F) -> i32 {
    f(7)
}

fn apply_to_3<F>(f: F) -> i32 where
// The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {
    f(3)
}


fn hello(name: &str) {
    println!("Hello, {}!", name);
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::collections::HashMap;

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Data can be accessed via the references and the original owner
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // Error! Can't borrow `point` as mutable because it's currently
    // borrowed as immutable.
    // let mutable_borrow = &mut point;
    // TODO ^ Try uncommenting this line

    // The borrowed values are used again here
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // The immutable references are no longer used for the rest of the code so
    // it is possible to reborrow with a mutable reference.
    let mutable_borrow = &mut point;

    // Change data via mutable reference
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Error! Can't borrow `point` as immutable because it's currently
    // borrowed as mutable.
    // let y = &point.y;
    // TODO ^ Try uncommenting this line

    // Error! Can't print because `println!` takes an immutable reference.
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ Try uncommenting this line

    // Ok! Mutable references can be passed as immutable to `println!`
    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // The mutable reference is no longer used for the rest of the code so it
    // is possible to reborrow
    let new_borrowed_point = &point;
    println!(
        "Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );

    let mut request = String::new();
    let mut tmp = String::new();
    let mut params = HashMap::new();

    params.insert(&tmp, &request);

    let five = Box::new(5);

    let reference_1 = &five;
    let reference_2 = reference_1;

    println!("1 {}", reference_1);
    println!("2 {}", reference_2);

    let mut s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice2 = &s[3..];

    s.clear();

    //println!("{}", slice);
}

fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

struct ThingOne {
    value: u32,
}

// Assume I can't change this signature to simply own thing_ref.  Maybe
// it's from third-party code.
struct ThingTwo {
    thing_ref: ThingOne,
}

fn test() -> ThingTwo {
    let thing1 = ThingOne { value: 1 };

    ThingTwo { thing_ref: thing1 }
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

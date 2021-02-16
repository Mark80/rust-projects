use std::fmt::Display;

struct ToDrop {}

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("DRRRRROOOOPPPP");
    }
}

#[derive(Debug, Copy)]
struct ToClone {
    x: i32,
}

impl Clone for ToClone {
    fn clone(&self) -> Self {
        ToClone { x: self.x.clone() }
    }
}

fn main() {
    {
        let to_drop = ToDrop {};
        println!("chiamo drop ... ");
    }

    println!("FINE");

    let bb = Box::new(5);
    in_box(bb, 7);

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    borrow_i32_2(stacked_i32);
    //borrow_i32_2(stacked_i32);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        //eat_box_i32(boxed_i32);
        // FIXME ^ Comment out this line

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.
    }

    // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    eat_box_i32(boxed_i32);

    //CLONE

    let to_clone = ToClone { x: 5 };

    take_ownership(to_clone);
    println!("{:?}", to_clone)
}

fn take_ownership(to_clone: ToClone) {
    println!("{:?}", to_clone)
}

fn in_box<T: Display>(mut bx: Box<T>, t: T) {
    *bx = t;
    println!("{}", bx);
}

// This function takes ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn borrow_i32_2(borrowed_i32: i32) {
    println!("This int is primitive: {}", borrowed_i32);
}

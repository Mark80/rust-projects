fn main() {
    let mut f = fibonacci();
    println!("{}", f() as i32);
    println!("{}", f() as i32);
    println!("{}", f() as i32);
    println!("{}", f() as i32);
    println!("{}", f() as i32);
}

fn fibonacci() -> Box<dyn FnMut() -> i32> {
    let mut i = 0;
    let mut j = 1;

    Box::new(move || {
        let r = i + j;
        i = j;
        j = r;
        r
    }
    )
}

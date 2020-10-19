#[derive(Debug)]
enum IPType {
    IP4(String),
    IP6(String),
}

impl IPType {
    fn show(&self) -> &str {
        "self"
    }
}

fn main() {
    route(IPType::IP4(String::from("127.0.0.0")));

    let some_int: Option<i32> = Some(5);
    let none: Option<i32> = None;

    fn sq(x: u32) -> Option<u32> { Some(x * x) }

    let result: Option<u32> = Some(2).and_then(sq);
    println!("{:?}", result);

    match some_int {
        Some(6) => println!("six"),
        Some(5) => println!("five"),
        _=> println!("hola"),
    }
}

fn route(ip: IPType) {
    println!("{}", ip.show())
}

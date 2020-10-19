use std::mem;
use crate::List::{Cons, Nil};

struct My{
    element : i32
}

enum List {
    Cons(i32,Box<List>),
    Nil,
}

fn main(){

    let list = Cons(2,Cons(3,Box::new(Cons(4,Box::new(Nil)))));

    let i = 45;
    let ibox = Box::new(45);
    let s = Box::new("eccomifsdfsadfsadf");
    let p1 = My{
        element : 5
    };

    let ar = [1,2,3,4,5];

    let string: String = String::from("ciao ciao ciao ciao ciao");

    println!("i take {}", mem::size_of_val(&i));
    println!("ibox take {}", mem::size_of_val(&ibox));
    println!("s take {}", mem::size_of_val(&s));
    println!("p1 take {}", mem::size_of_val(&p1));
    println!("ar take {}", mem::size_of_val(&ar));
    println!("string take {}", mem::size_of_val(&string));

    let p3 = *s;
    println!("p3 take {}", mem::size_of_val(&p3));


}
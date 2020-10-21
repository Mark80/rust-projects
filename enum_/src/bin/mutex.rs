
use std::sync::Mutex;

fn main() {

    let m = Mutex::new(6);

    {
        let mut number  = m.lock().unwrap();
        *number = 8;
    }

    println!("m = {:?}", m);

}
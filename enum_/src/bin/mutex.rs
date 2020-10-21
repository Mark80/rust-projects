use std::sync::{Mutex,Arc};
use std::thread;

fn main() {

    let m = Mutex::new(6);

    {
        let mut number  = m.lock().unwrap();
        *number = 8;
    }

    println!("m = {:?}", m);


    let counter = Arc::new(Mutex::new(0));
    let mut threads = vec![];


    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let th =thread::spawn(move || {
            let mut val = counter.lock().unwrap();
            *val += 1;
        });

        threads.push(th);
    }


    for th in threads {
        th.join().unwrap();
    }

    println!("{:?}", *counter.lock().unwrap());


}
use std::thread;
use std::time::Duration;


fn main() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(500));
        }
    });


    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(500));
    }

    handle.join();

    let v = vec![1,1,1,1];

    let handle2 = thread::spawn(move || {
        println!("vector {:?}", v);
    });

    handle2.join().unwrap();

}
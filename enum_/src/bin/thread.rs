use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;
use std::sync::{mpsc, MutexGuard};
use std::sync::Mutex;


fn main() {

        let joiner: JoinHandle<()> = thread::spawn(move ||{
            for n in 1..9 {
                println!("thread : {}", n);
                thread::sleep(Duration::from_millis(100));
                println!("thread end : {}", n);
            }

        } );


    for n in 1..5 {
        println!("main : {}", n);
        thread::sleep(Duration::from_millis(1));
    }

    joiner.join();


    let (tx,rx) = mpsc::channel();

    let jS = thread::spawn(move || {
        println!("sending");
        tx.send(5);
    });

    thread::sleep(Duration::from_millis(1000));

    let jR = thread::spawn(move || {
        println!("sending");
        let received = rx.recv().unwrap();
        println!("Received {}", received);

    });


    jS.join();
    jR.join();


    //////////////
    let m = Mutex::new(5);

    {
        let mut num: MutexGuard<i32> = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);




}
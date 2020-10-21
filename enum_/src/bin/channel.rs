use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {


    let (transmitter, receiver) = mpsc::channel();


    let trasmitter_clone = mpsc::Sender::clone(&transmitter);

    thread::spawn(move || {
        let message = String::from("messaggio");
        transmitter.send(message).unwrap();
    });

    thread::spawn(move || {
        let message = String::from("messaggio2");
        trasmitter_clone.send(message).unwrap();
    });


    for received in receiver {
        println!("Got: {}", received);
    }



    thread::sleep(Duration::from_millis(600));
}
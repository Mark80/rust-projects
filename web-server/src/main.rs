use std::fs;
use std::io::{Read, Write};
use std::net::*;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{mpsc, Arc, Mutex};
use  std::sync::mpsc::{Sender,Receiver};

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    let thread_pool = ThreadPool::new(4).unwrap();

    for stream in listener.incoming() {
        let connection = stream.unwrap();

        thread_pool.execute(|| {
            handling(connection)
        });
        //handling(connection);
    }
    println!("Hello, world!");
}

fn handling(mut connection: TcpStream) {
    let mut buffer = [0; 1024];
    let request = connection.read(&mut buffer);
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1";

    let (contents, status) = if buffer.starts_with(get) {
        (fs::read_to_string("src/hello.html").unwrap(), "HTTP/1.1 200 OK")
    } else {
        (fs::read_to_string("src/404.html").unwrap(), "HTTP/1.1 404 NOT FOUND")
    };


    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );


    connection.write(response.as_bytes()).unwrap();
    connection.flush().unwrap();
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    threads: Vec<Worker>,
    size: usize,
    sender : Sender<Job>,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.threads {
            if let Some(thread) =  worker.thread.take(){
                 println!("Shooting down thread: {}",worker.id);
                 thread.join().unwrap();
            }
        }

    }
}

impl Worker {
    fn new(id: usize, receiver : Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread: JoinHandle<()> = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });
        Worker { id, thread :Some(thread) }
    }
}

impl ThreadPool {

    fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let mut workers: Vec<Worker> = Vec::with_capacity(size);
            let (sender,rx) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(rx));

            for id in 0..size {
                workers.push(Worker::new(id,Arc::clone(&receiver)));
            }

            Ok(ThreadPool {
                threads: workers,
                size,
                sender
            })
        } else {
            Err(PoolCreationError)
        }
    }

    fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

#[derive(Debug)]
struct PoolCreationError;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn connecting_to_socket() {
        let listener = TcpListener::bind("localhost:8080").unwrap();

        for stream in listener.incoming() {
            stream.unwrap();
            println!("Connection establish!!");
        }
    }
}

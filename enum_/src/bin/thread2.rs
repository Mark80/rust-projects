#![feature(generators)]
#![feature(generator_trait)]
#![feature(impl_trait_in_bindings)]
// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

// I AM NOT DONE

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::ops::Generator;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {

    let mut gen:impl Generator = || {
       let xs  = vec![1,2,3];
        let mut sum = 0;

        for v in xs.iter() {
            sum += v;
            yield sum;
        }
    };

    // let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    // let status_shared = status.clone();
    // thread::spawn(move || {
    //     for _ in 0..10 {
    //         thread::sleep(Duration::from_millis(250));
    //         let mut status_2 = status_shared.lock().unwrap();
    //         status_2.jobs_completed += 1;
    //         println!("{}", status_2.jobs_completed);
    //     }
    // });
    // while status.lock().unwrap().jobs_completed < 10 {
    //     println!("waiting... ");
    //     thread::sleep(Duration::from_millis(500));
    // }
}



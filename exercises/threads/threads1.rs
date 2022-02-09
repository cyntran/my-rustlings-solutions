// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Mutex,Arc};
use std::thread;
use std::time::Duration;

// lock values accessed concurrently via Mutex
struct JobStatus {
    jobs_completed: Mutex<u32>,
}

fn main() {
    // Arc allows you to share this Status struct across multiple threads 
    let status = Arc::new(JobStatus { 
        jobs_completed: Mutex::new(0) 
    });

    // bumps up arc's reference count (out of scope, decreases count)
    // necessary to know how many references of Status exists across
    // the threads to know how to deallocate it. 
    let status_shared = status.clone();

    // we execute code within the closure inside a new thread
    // 'move' gives the captured values to the closure
    thread::spawn(move || {
        for _ in 0..10 {
            // the lock on status gets dropped outside this scope
            let mut count = status_shared.jobs_completed.lock().unwrap();
            *count += 1;
            thread::sleep(Duration::from_millis(250));
        }
    });

    while *status.jobs_completed.lock().unwrap() < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}

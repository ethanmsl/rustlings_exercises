// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed


use std::sync::Arc;
use std::thread;
use std::time::Duration;
// use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

struct JobStatus {
    // // for use with Mutex
    // jobs_completed: u32,

    // for use with AtomicU32
    jobs_completed: AtomicU32,
}

fn main() {
    // // Adding a mutex surround to JobStatus
    // let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status = Arc::new(JobStatus {
        jobs_completed: AtomicU32::new(0),
    });
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(2500));
            // TODO: You must take an action before you update a shared value
            // // aquiring lock on shared status
            // let mut muter = status_shared.lock().unwrap();
            // muter.jobs_completed += 1;

            // for use with AtomicU32
            status_shared.jobs_completed.fetch_add(1, Ordering::SeqCst)
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        //  ^ they all run for the same amount of time, concurrently
        // // for use with Mutex
        // println!("jobs completed {}", status.lock().unwrap().jobs_completed);
        //
        // for use with AtomicU32
        println!("jobs completed {:?}", status.jobs_completed);
    }
}

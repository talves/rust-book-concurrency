use std::thread;
use std::time::Duration;

pub fn spawn_threads(join_thread: bool) {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    if join_thread {
        handle.join().unwrap() // join waits for threads to complete or checks if already done
    };

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

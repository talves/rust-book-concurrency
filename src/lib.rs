use std::rc::Rc;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
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

pub fn move_closures_threads() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

pub fn message_threads() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("Sending the message: {}", val);
        thread::sleep(Duration::from_millis(2));
        tx.send(val).unwrap(); // val has been moved into send because it takes ownership
    });

    let received = rx.recv().unwrap(); // the value of `val` is sent to recv who now has ownership.
    println!("Got: {}", received);
}

pub fn multi_message_threads() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("...."),
            String::from("multiple"),
            String::from("messages"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn multi_producers_threads() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("tx: hi"),
            String::from("tx: from"),
            String::from("tx: the"),
            String::from("tx: thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("tx1: more"),
            String::from("tx1: messages"),
            String::from("tx1: for"),
            String::from("tx1: you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("{}", received);
    }
}

pub fn shared_state_concurrency() {
    // https://doc.rust-lang.org/book/ch16-03-shared-state.html#the-api-of-mutext
    println!("/nMutex");
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // aquire the lock and returns a MutexGuard (smart pointer)
        println!("num = {:?}", num);

        *num = 6;
    }

    println!("m = {:?}", m);

    // Share the Mutex<T> between multiple threads using Arc (atomically reference counted)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

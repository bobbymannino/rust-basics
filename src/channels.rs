use std::{sync::mpsc, thread, time::Duration};

pub fn channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = 221279;
        thread::sleep(Duration::from_secs(1));
        // tx is clonable so you can use it in multiple threads
        tx.send(val).unwrap();
    });

    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_millis(100));
            println!("Still waiting...");
        }
    });

    // waits for a value to be sent, blocks the main thread until such
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}

pub fn multiple_vals() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            thread::sleep(Duration::from_secs(1));
            tx.send(val).unwrap();
        }
    });

    // will block the main thread also, it knows then its stopped because tx will go out of scope
    for received in rx {
        println!("Got: {received}");
    }
}

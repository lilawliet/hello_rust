use std::{sync::mpsc, time::Duration, thread};


fn producer_consumer() {
    let (tx, rx) = mpsc::channel();

    let producer_handle = thread::spawn(move || {
        for i in 0..10 {
            tx.send(format!("Produced: {}", i)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    let consumer_handle = thread::spawn(move || {
        for received in rx {
            println!("Received: {}", received);
        }
    });

    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}

fn main() {
    producer_consumer();
}
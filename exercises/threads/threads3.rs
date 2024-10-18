// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Mutex<Queue>>, tx: mpsc::Sender<u32>) {
    let tx1 = tx.clone(); // Clone the sender for the first thread
    let tx2 = tx; // Use the original sender for the second thread

    // Thread for sending the first half of the queue
    let qc1 = Arc::clone(&q);
    let t1 = thread::spawn(move || {
        let queue = qc1.lock().unwrap();
        for val in &queue.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Thread for sending the second half of the queue
    let qc2 = Arc::clone(&q);
    let t2 = thread::spawn(move || {
        let queue = qc2.lock().unwrap();
        for val in &queue.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for both threads to finish
    t1.join().unwrap();
    t2.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Mutex::new(Queue::new()));

    // Start sending values in separate threads
    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    // Receive values in the main thread
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("Total numbers received: {}", total_received);
    assert_eq!(total_received, 10);
}





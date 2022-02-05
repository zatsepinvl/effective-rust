use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    //threads();
    //channels();
    //mutex();
    sync_and_send();
}

fn threads() {
    let v = vec![1, 2, 3, 4, 5];
    thread::spawn(move || {
        println!("Worker: {:?}", v);
        thread::sleep(Duration::from_secs(1));
        println!("Worker: finish");
    }).join();
}

fn channels() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
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
        println!("Got {}", received);
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

fn mutex() {
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

    println!("Result = {:?}", counter.lock().unwrap());
}

fn sync_and_send() {
    use std::marker::Send;
    use std::marker::Sync;
}




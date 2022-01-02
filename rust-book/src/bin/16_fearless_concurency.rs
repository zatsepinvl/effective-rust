use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    //threads();
    channels();
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

fn threads() {

    let v = vec![1, 2, 3, 4, 5];
    thread::spawn(move || {
        println!("Worker: {:?}", v);
        thread::sleep(Duration::from_secs(1));
        println!("Worker: finish");
    }).join();
}


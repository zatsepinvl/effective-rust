use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn up(&mut self) {
        self.y += 1;
    }
}

fn main() {
    let mut point = Point { x: 1, y: 2 };
    let counter = Arc::new(point);
    let mut handles = vec![];

    for _ in 0..10 {
        let mut counter = counter.clone();
        let handle = thread::spawn(move || {

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {:?}", point);
}

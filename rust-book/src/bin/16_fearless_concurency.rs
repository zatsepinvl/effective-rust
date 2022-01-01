use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    thread::spawn(move || {
        println!("Worker: {:?}", v);
        thread::sleep(Duration::from_secs(1));
        println!("Worker: finish");
    }).join();
}

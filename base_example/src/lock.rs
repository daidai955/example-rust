use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn run() {
    println!("------ lock.rs ------");

    let data = Arc::new(Mutex::new(Vec::new()));
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let data = Arc::clone(&data);
        let tx = tx.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data.push(i);
            tx.send(()).unwrap();
        });
    }

    for _ in 0..3 {
        rx.recv().unwrap();
    }
}

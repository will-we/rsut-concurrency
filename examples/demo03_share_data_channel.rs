use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel::<String>();

    thread::spawn(move || {
        for i in 1..10 {
            tx.send(i.to_string()).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    while let Ok(received) = rx.recv() {
        println!("Got: {received}");
    }
}

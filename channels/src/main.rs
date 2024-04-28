use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("Hi there!");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("{}", received);
}
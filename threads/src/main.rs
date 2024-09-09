use std::{sync::mpsc, thread, time::Duration};

fn main() {
    thread::spawn(|| {
        println!("Thread spawned...");
    });
    println!("Executing main...");
    // thread::sleep(Duration::from_secs(2));

    let message_to_send = String::from("Duck");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(message_to_send).expect("transmitter failed");
        // println!("Message sent : {}", message_to_send);
    });

    thread::sleep(Duration::from_secs(1));

    let message_received = rx.try_recv().expect("receive failed");

    assert_eq!("Duck", message_received);

    multi_threads();
    multi_prods();
}

fn multi_threads() {
    let (tx, rx) = mpsc::channel();

    let messages_to_send = vec![
        "a", "b", "c", "d", "e", "a", "b", "c", "d", "e", "a", "b", "c", "d", "e", "a", "b", "c",
        "d", "e", "a", "b", "c", "d", "e", "a", "b", "c", "d", "e",
    ];

    thread::spawn(move || {
        for msg in messages_to_send {
            tx.send(msg).expect("failed to send");
            // thread::sleep(Duration::from_secs(1));
        }
    });

    let mut received_messages: Vec<&str> = vec![];

    for received in rx {
        received_messages.push(received);
    }

    // thread::sleep(Duration::from_secs(0));

    assert_eq!(
        received_messages,
        vec![
            "a", "b", "c", "d", "e", "a", "b", "c", "d", "e", "a", "b", "c", "d", "e", "a", "b",
            "c", "d", "e", "a", "b", "c", "d", "e", "a", "b", "c", "d", "e"
        ]
    );
}

fn multi_prods() {
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    let messages_to_send = vec![
        "a", "b", "c", "d", "e", "a", "b", "c", "d", "e", "a", "b", "c", "d", "e", "a", "b", "c",
        "d", "e", "a", "b", "c", "d", "e", "a", "b", "c", "d", "e",
    ];

    let messsages_to_send_clone = messages_to_send.clone();

    thread::spawn(move || {
        for msg in messages_to_send {
            tx.send(msg).expect("failed to send");
        }
    });

    thread::spawn(move || {
        for msg in messsages_to_send_clone {
            tx_clone.send(msg).expect("failed to send");
        }
    });

    let mut received_messages: Vec<&str> = vec![];

    for received_msgs in rx {
        received_messages.push(&received_msgs);
    }

    assert_eq!(
        vec![
            "a", "b", "c", "d", "e", "a", "b", "c", "d", "e", "a", "b", "c", "d", "e", "a", "b",
            "c", "d", "e", "a", "b", "c", "d", "e", "a", "b", "c", "d", "e", "a", "b", "c", "d",
            "e", "a", "b", "c", "d", "e", "a", "b", "c", "d", "e", "a", "b", "c", "d", "e", "a",
            "b", "c", "d", "e", "a", "b", "c", "d", "e",
        ]
        .len(),
        received_messages.len()
    );
}

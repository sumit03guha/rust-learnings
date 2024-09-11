use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Mutex::new(0);
    println!("Counter init: {:?}", counter);

    // thread::spawn(move || {
    //     let mut num = value.expect("failed to take lock");
    //     *num += 1;
    // });

    {
        let mut value = counter.lock().expect("failed to take lock");
        *value += 10;
    }

    println!("Counter : {:?}", counter);

    sharing_mutex_between_multiple_threads();
}

fn sharing_mutex_between_multiple_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..20 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().expect("failed to take lock");
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("failed to join handles");
    }

    println!("Counter : {:?}", counter);
}

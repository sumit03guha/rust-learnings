use std::sync::Mutex;



fn main() {
    let m = Mutex::new(123);

    {
        let mut num = m.lock().unwrap();
        *num = 11;
    }
}
static COUNTER: i32 = 1;

fn main() {
    unsafe {
        println!("{}", COUNTER);
    }
}

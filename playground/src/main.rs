use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 123;
    let my_box = MyBox::new(123);

    assert_eq!(x, *my_box);

    let sample = String::from("ABcdef");
    let sliced_sample = &sample[..4];

    println!("{}", sliced_sample);
    
    let mut data = 12;

    let first = &mut data;
    
    let second = &mut data;

    // println!("{} {}", first, second);

    let mut scores = vec![1, 2, 3];
    let score = &scores[0];
    scores.push(4);

    // print!("{score}");

    let mut x = 5;
    let y = &x;
    let z = &mut x;
    // println!("y: {}", y);

}

use std::ops::Deref;

#[derive(Debug)]
struct MyDrop(i32);

impl Drop for MyDrop {
    fn drop(&mut self) {
        println!("Dropping --> {:?}", self.0)
    }
}

impl Deref for MyDrop {
    type Target = MyDrop;
    fn deref(&self) -> &Self::Target {
        &self
    }
}

impl MyDrop {
    fn new(x:i32) -> Self {
        MyDrop(x)
    }
}

fn main() {
    let mydrop = MyDrop::new(123);
    drop(mydrop);
    
    let another_mydrop = MyDrop::new(123123);

    let _reference_to_another_mydrop = &another_mydrop;

    // drop(another_mydrop);

    // println!("{:?}", *_reference_to_another_mydrop);

    println!("Created");

    let rc_ref = &_reference_to_another_mydrop;

}

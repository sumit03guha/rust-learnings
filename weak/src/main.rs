use std::{rc::{Rc, Weak}, borrow::BorrowMut, cell::RefCell};

fn main() {
    let val = String::from("Sumit");
    let a = RefCell::new(Weak::new());
    *a.borrow_mut() = Rc::downgrade(&Rc::new(val));

    let val2 = String::from("Sumit");

    let boxx = RefCell::new(val2);
    *boxx.borrow_mut() = String::from("sss");

    println!("{:#?}",boxx);
}

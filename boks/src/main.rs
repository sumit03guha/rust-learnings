enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

use std::ops::Deref;

struct MyBox<T,U>(T,U);

impl<T,U> MyBox<T,U> {
    fn new(x:T, y:U) -> MyBox<T,U> {
        MyBox(x, y)
    }
}

impl<T,U> Deref for MyBox<T,U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}


fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let mybox = MyBox::new(13, 12);

    assert_eq!(*mybox, 12);

}
enum ConsList {
    Cons(i32, Box<ConsList>),
    Nil
}

use crate::ConsList::{Cons,Nil};

fn main() {
    let cons_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

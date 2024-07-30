use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}


fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("The strong rc count at line 13 is {}", Rc::strong_count(&a));
    println!("The weak rc count at line 14 is {}", Rc::weak_count(&a));

    {
        let b = Cons(3, Rc::clone(&a));
        println!("The strong rc count at line 18 is {}", Rc::strong_count(&a));
        println!("The weak rc count at line 19 is {}", Rc::weak_count(&a));

    }
    println!("The strong rc count at line 22 is {}", Rc::strong_count(&a));
    println!("The weak rc count at line 23 is {}", Rc::weak_count(&a));
    
    let c = Cons(4, Rc::clone(&a));

    println!("The strong rc count at line 27 is {}", Rc::strong_count(&a));
    println!("The weak rc count at line 228 is {}", Rc::weak_count(&a));

    // let a = Cons(1, &Cons(2, &Cons(3, &Nil)));
    // let b = Cons(4, &a);
    // let c = Cons(5, &a);
}
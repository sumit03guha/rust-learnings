use std::cell::RefCell;
use std::rc::Rc;

enum ConsList {
    Cons(i32, Box<ConsList>),
    Nil,
}

enum RcConsList {
    Cons(i32, Rc<RcConsList>),
    Nil
}

#[derive(Debug)]
enum RefConsList {
    Cons(Rc<RefCell<i32>>, Rc<RefConsList>),
    Nil
}

use crate::ConsList::{Cons, Nil};
use crate::RcConsList::{Cons as RcCons, Nil as RcNil};
use crate::RefConsList::{Cons as RefCons, Nil as RefNil};

fn main() {
    let list_a = Cons(1, Box::new(Cons(2, Box::new(Cons(3,Box::new(Nil))))));

    let list_b = Cons(10, Box::new(list_a));

    let list_c = Cons(20, Box::new(list_b));

    // let list_d = Cons(30, Box::new(list_a));

    let list_a_rc = Rc::new(RcCons(12, Rc::new(RcCons(123, Rc::new(RcCons(1234, Rc::new(RcNil)))))));

    let list_b_rc = RcCons(100, Rc::clone(&list_a_rc));

    let list_c_rc = RcCons(100, Rc::clone(&list_a_rc));

    println!("Strong count : {}", Rc::strong_count(&list_a_rc));
    println!("Weak count : {}", Rc::weak_count(&list_a_rc));

    let value = Rc::new(RefCell::new(1));

    let ref_cell_list_a = Rc::new(RefCons(Rc::clone(&value), Rc::new(RefNil)));

    println!("ref_cell_list_a before : {:?}", ref_cell_list_a);

    *value.borrow_mut() += 10;

    println!("ref_cell_list_a after : {:?}", ref_cell_list_a);

    let mut x = 5;

    let y = &x;

    let x_ref_cell = RefCell::new(5);

    let x_rc = Rc::new("10");

    let y = x_ref_cell.borrow_mut();

    println!("y : {}", y);

    let val = Rc::new(123);

    let val2 = *val.abs();

}

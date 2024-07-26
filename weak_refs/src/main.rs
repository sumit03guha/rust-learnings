use std::{borrow::Borrow, cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
struct Node {
    value:i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

fn main() {
    let leaf = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())
    });

    // println!("leaf before branch- weak count: {} and strong count: {}", Rc::weak_count(&leaf), Rc::strong_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 10,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new())
        });
    
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // println!("leaf after branch- weak count: {} and strong count: {}", Rc::weak_count(&leaf), Rc::strong_count(&leaf));
        // println!("branch- weak count: {} and strong count: {}", Rc::weak_count(&branch), Rc::strong_count(&branch));

        // println!("leaf parent : {:?}", leaf.parent.borrow().upgrade());
        let upgraded = leaf.parent.borrow().upgrade();
        // println!("leaf after upgrade- weak count: {} and strong count: {}", Rc::weak_count(&leaf), Rc::strong_count(&leaf));



    }

    // println!("leaf outside- weak count: {} and strong count: {}", Rc::weak_count(&leaf), Rc::strong_count(&leaf));

    let initial = Rc::new(123);

    println!("Initial counts : {} , {}", Rc::strong_count(&initial), Rc::weak_count(&initial));

    let weak_pointer = Rc::downgrade(&initial);

    println!("Initial counts : {} , {}", Rc::strong_count(&initial), Rc::weak_count(&initial));

    // drop(initial);

    let back_strong = weak_pointer.borrow().upgrade().unwrap();

    println!("Initial counts : {} , {}", Rc::strong_count(&initial), Rc::weak_count(&initial));

}

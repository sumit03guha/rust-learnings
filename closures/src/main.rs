#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        // sort_operations.push(value);  // Problematic line
        r.width
    });

    println!("{:#?}", list);

    fn closure_once<F>(f: F)
        where
        F: 
        // Fn() + 
        FnOnce() + 
        // FnMut()
    {
        f()
    }

    let a = String::from("s");

    let x = move || {drop(value)};

    closure_once(x);

    // println!("{a}")
    
}

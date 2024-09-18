trait Draw {
    fn draw(&self) {}
}

/*
* using generics
*/

struct Screens<T: Draw> {
    collections: Vec<T>,
}

impl<T> Screens<T>
where
    T: Draw,
{
    fn draw(&self) {
        for component in self.collections.iter() {
            component.draw();
        }
    }
}

/*
* using trait objects
*/

struct Sketch {
    collections: Vec<Box<dyn Draw>>,
}

impl Sketch {
    fn run(&self) {
        for component in self.collections.iter() {
            component.draw();
        }
    }
}

fn main() {
    struct Button {
        width: u32,
        height: u32,
        label: String,
    }

    struct InputBox {
        width: u32,
        height: u32,
        label: String,
    }

    impl Draw for Button {
        fn draw(&self) {}
    }

    impl Draw for InputBox {
        fn draw(&self) {}
    }

    let sketch = Sketch {
        collections: vec![
            Box::new(Button {
                width: 23,
                height: 34,
                label: String::from("OK"),
            }),
            Box::new(InputBox {
                width: 10,
                height: 4,
                label: String::from("Please enter..."),
            }),
        ],
    };

    sketch.run();

    let screens = Screens {
        collections: vec![Button {
            width: 13,
            height: 12,
            label: String::from("Ok"),
        }],
    };
}

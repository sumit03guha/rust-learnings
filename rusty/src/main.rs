// use std::collections::HashMap;

struct MyBox(i32);

struct Person {
    name:String,
    age:i32
}

trait Getter {
    fn get_age(&self) -> &i32;

    fn get_name(&self) -> &str;
}

trait Setter {
    fn set_age(&mut self, age: &i32);

    fn set_name(&mut self, name: &str);
}

impl Getter for Person {
    fn get_age(&self) -> &i32 {
        &self.age
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Setter for Person {
    fn set_age(&mut self, _age: &i32) {
        self.age = *_age
    }

    fn set_name(&mut self, _name: &str) {
        self.name = _name.to_string()
    }
}

enum Colour {
    Red,
    Blue,
    Yellow
}

fn print_colour(input_colour: &Colour) {
    match input_colour {
        Colour::Red => println!("RED!"),
        _ => println!("gg")
    }

}

fn main() {
    // print_colour(&Colour::Red)
    let mut person = Person {
        name: String::from("Sumit Guha"),
        age: 26
    };

    let age = Getter::get_age(&person);
    println!("{}", age);

    Setter::set_age(&mut person, &21);

    let age = Getter::get_age(&person);
    println!("{}", age);
}

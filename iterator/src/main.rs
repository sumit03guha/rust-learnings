fn main() {
    let mut array_of_str: Vec<String> = vec![String::from("a"),String::from("b"),String::from("c")];

    let mut array_iter = array_of_str.iter();

    for element in array_iter {
        print!("{element}")
    }

    for element in array_of_str.iter() {
        println!("{}", element);
        drop(element)
    }

    // println!("{:?}", array_of_str)

    let closure = |x| { x + 1 };

    let v1 = vec![1,2,3,4];

    let v1_iter = v1.iter();

    let v2 = v1_iter.map(closure).collect::<Vec<i32>>();


}

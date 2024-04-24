fn main() {

    let x: &str = "hello";

    fn hello() -> i32 {
        let guess = "3";

        match guess.trim().parse() {
            Ok(num) if num == 1 => return num,
            Ok(_) => panic!("deada"),
            Err(_) => panic!("dead"),
        };
    }

    let h = hello();
    println!("{}", h);
}
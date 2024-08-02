fn main() {
    let user_input = 100;

    let res = check_it_out(user_input);

    println!("result: {}", res.unwrap());
}

fn check_it_out(input: u32) -> Result<i8, String> {
    if input == 69 {
        return Ok(100);
    } else {
        return Err(String::from("you lose!"));
    }
}

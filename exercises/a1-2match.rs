use std::io;

fn main() {
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(n@ 0) => {
            panic!("input length is ZERO!")
        }
        Ok(_) => {
            name.pop();
            if name_integrity_check(&name) {
                name = name.trim().parse::<String>().unwrap();
                println!("Hello {}.", name)
            } else {

            }
        }
        Ok(n@ 1) => {
            panic!("input is empty(only '\n')")
        }
        Err(_) => (),
    }
}
fn name_integrity_check(chars: &str) -> bool {
    // check with trailing \n removed
    let mut is_valid = true;
    for ch in chars.chars() {
        if !(ch.is_whitespace() || ch.is_alphabetic() || ch=='.'){
            panic!("Name should contains only space, ',', and alphabetic characters, but not: {}.", ch);
            is_valid = false;
            break
        }
    }
    is_valid
}
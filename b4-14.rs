use std::io;
fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error read line");
    buffer.pop();
    // let n = buffer.parse::<u32>().expect("input is not valid integer");

    println!("{}", buffer.chars().rev().collect::<String>())



}
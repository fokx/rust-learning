use std::io::{Read, BufRead, stdin};

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("error read line");
    buffer.pop();
    let n = buffer.parse::<u32>().expect("input is not valid integer");
    let mut vec:Vec<u64> = Vec::new();//= Vec::with_capacity(n as usize);

    // stdin()::read_line() APPEND to `buffer` string instead of EMPTY it first!!
    // io::empty();
    // assert!(buffer.is_empty(), "Buffer is not empty, but {}.", buffer);
    buffer.clear();
    // or
    // buffer = "".parse().unwrap();
    stdin().read_line(&mut buffer).expect("error read the second line");

    let split = buffer.split_whitespace();

    for s in split {
        vec.push(s.parse::<u64>().expect("error parse number"));
    }
    let mut space = "";
    for num in vec.iter().rev(){
        print!("{}{}", space, num);
        space = " ";
    }

}
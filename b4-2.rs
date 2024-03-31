use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error read line");

    let split = buffer.split_whitespace();
    let mut vec = Vec::new();
    for s in split {
        vec.push(s.parse::<i64>().expect("element is not valid number"));
    }
    assert_eq!(vec.pop(), Some(-9999), "the number sequence is not properly ended(with -9999)");
    println!("{}",vec.iter().max().expect("error got maximum value"))

}



use std::io;
use std::io::Read;
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error read input");
    let split = buffer.split_whitespace();

    // Hashset member is unique
    // let sequence_of_char:HashSet<&str> = split.collect();
    // let split_size = sequence_of_char.len() ;

    let mut split_size = 0;
    for s in split.clone() {
        split_size += 1;
    }
    assert_eq!(split_size, 2, "Only 2 number expected, got {}.", split_size);

    let mut vec = Vec::new();
    for s in split.clone() {
        vec.push(s.parse::<f64>().expect("not valid number"));
    }

    assert_eq!(vec.len(), 2, "Only 2 number expected, got {}.", split_size);
    println!("{}", vec[0] * vec[1]);
}
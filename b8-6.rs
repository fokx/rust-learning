use std::io;
use core::fmt;
use std::fmt::Formatter;

fn main() {
    let mut vec = read_line_to_vec();
    /*
    do not use vec!(vec!()) to initialize, which will introduce a blank new value in vec_2d!
    for element in vec {
            println!("{}", element);
        }
     */
    let mut vec_2d: Vec<Vec<String>> = vec!();
    while !(&vec[0] == "0" && &vec[1] == "0" && &vec[2] == "0") {
        vec_2d.push(vec.clone());
        vec.clear();
        vec = read_line_to_vec();
    }

    let mut vec_person:Vec<Person> = vec!();
    for  vec in vec_2d {
        vec_person.push(
            Person {
                name:vec[0].clone(),
                gender:vec[1].clone(),
                age:vec[2].parse().expect("not valid age")
            }
        );
    }

    for person in vec_person.iter().rev() {
        println!("{}", person);
    }
}

fn read_line_to_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error read line");
    let split = buffer.split_whitespace();
    let vec = split
        .map(|x|
            x.parse::<String>()
                .expect("error parse element to String"))
        .collect::<Vec<_>>();
    assert_eq!(vec.len(), 3, "expect 3 element in one line, got {}", vec.len());
    vec
}

// #[derive(Debug)]
struct Person {
    name: String,
    gender: String,
    age: i32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.gender, self.age)
    }
}
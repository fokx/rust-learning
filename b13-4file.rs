use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    println!("{}", env::current_dir().unwrap().display());
    let mut file = File::open("-/.bashrc").unwrap();
    let mut contents = String::new();
    // cannot use the `?` operator in a function that returns `()`
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents)
}
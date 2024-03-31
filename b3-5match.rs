use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error read line");
    buffer.pop();
    let mut num = buffer.parse::<u32>().expect("input is not valid integer");
    num /= 10;
    let info = match num {
        0 => 0,
        1..=5 => 1,
        6 => 2,
        7 => 3,
        8 => 4,
        9|10 => 5,
        _ => {panic!("input number not in legal range([0,100])")}
    };
    println!("{}", info)

}
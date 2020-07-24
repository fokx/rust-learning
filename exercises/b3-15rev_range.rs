use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error read line");
    buffer.pop();
    let n = buffer.parse::<u32>().expect("input is not valid integer");
    for i in (1..n+1).rev(){
        for j in 1..i+1 {
            print!("{}*{}={}", i, j, i*j);
            if !(j == i-1) {
                print!(" ")
            }
        }
        println!()
    }
}
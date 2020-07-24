use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error read line");
    buffer.pop();
    let n = buffer.parse::<u64>().expect("input is not valid integer");

    for i in 0..n+1{
        print!("{}", fibonacci(i));
        if i != n {
            print!(" ");
        } else {
            println!();
        }
    }

}
fn fibonacci(n: u64) -> u64 {
    match n  {
        0 => 0,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2)
    }
}
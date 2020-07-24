use std::io;
use std::io::Read;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error read line");
    buffer.pop();
    let year = buffer.parse::<u32>().expect("input is not valid integer");
    let is_leap_year :bool;
    is_leap_year = if year % 100 == 0 {
        if year % 400 == 0 {
            true
        } else {
            false
        }
    } else {
        if year % 4 == 0 {
            true
        } else {
            false
        }
    };
    if is_leap_year{
        println!("IsLeapYear")
    } else {
        println!("NotLeapYear")
    }

}
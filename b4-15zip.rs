use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error read line");
    let split = buffer.split_whitespace()
        .map(|x| x.parse::<String>().expect("parse result is not string"))
        // .map(|x| x.parse::<String>().expect("parse result is not string"))
        .collect::<Vec<String>>();
    assert_eq!(split.len(), 2, "length {} != 2", split.len());
    // let vec1 = split[0].chars().collect::<Vec<_>>();
    // let vec2 = split[1].chars().collect::<Vec<_>>();

    let mut to_print= "0";
    let zipped = split[0].chars().zip(split[1].chars());
    for (x, y) in zipped {
        println!("x:{}, y:{}", x ,y);
        if (x as i32) < (y as i32) {
            to_print = "-1";
            break;
        } else if (x as i32) > (y as i32) {
            to_print = "1";
            break;
        } else {
            to_print = "0";
        };
    }
    println!("{}", to_print);
}
// iterate by row
use std::io;
fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error read line");
    let vec = buffer
        .split_whitespace()
        .map(|x|
            x.parse::<i32>()
                .expect("error parse to integer"))
        .collect::<Vec<i32>>();
    assert_eq!(vec.len(), 2, "expect only 2 input num");
    // n row num, m column num
    let n = vec[0];
    let m = vec[1];
    assert!(n > 0);
    assert!(m > 0);
    let mut data = vec![vec![0; m as usize]; n as usize];
    // let mut data:[[i32;m];n] = [[0;m];n];
    for i in 0..n {
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("error read line");
        let vec = buffer
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect(" item not legal integer"))
            .collect::<Vec<i32>>();
        assert_eq!(vec.len(), m as usize, "expect {} inputs", m);
        data[i as usize] = vec;

    }

    for row in data {
        println!("{}", row.iter().max().expect("cannot got maximum value in a row"))
    }
}
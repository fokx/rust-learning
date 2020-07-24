use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error read line");
    buffer.pop();
    let x = buffer.parse::<f64>().expect("input is not valid float");
    /*
    Returns the sine of an f64.
    The stabilized version of this intrinsic is std::f64::sin
     */
    // #![feature(core_intrinsics)]
    // use std::intrinsics::{sinf64, sinf32};
    // print!("{}", sinf64(x));
    println!("{}", anyfn(x_times_x, x));
}

fn anyfn(fx: fn(f64) -> f64, x: f64) -> f64 {
    return fx(x);
}

fn x_times_x(x: f64) -> f64 {
    x * x
}

use std::io::stdin;

fn main(){
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("error read line");
    buffer.pop();
    let c = buffer.parse::<char>().expect("error parse input to ASCII");
    // print ASCII
    println!("{}", c as u32)
    /*
     the only appropriate formatting traits are:
          - ``, which uses the `Display` trait
          - `?`, which uses the `Debug` trait
          - `e`, which uses the `LowerExp` trait
          - `E`, which uses the `UpperExp` trait
          - `o`, which uses the `Octal` trait
          - `p`, which uses the `Pointer` trait
          - `b`, which uses the `Binary` trait
          - `x`, which uses the `LowerHex` trait
          - `X`, which uses the `UpperHex` trait
     */
}
// convert int to char similarly:
//    let num = 65 as char;
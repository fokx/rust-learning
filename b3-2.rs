use std::io::stdin;
/* May not comile!
warning: floating-point types cannot be used in patterns
  --> src/main.rs:10:23
   |
10 |     let info = if let 0.0 = num {
   |                       ^^^
   |
   = note: `#[warn(illegal_floating_point_literal_pattern)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>

warning: floating-point types cannot be used in patterns
  --> src/main.rs:10:23
   |
10 |     let info = if let 0.0 = num {
   |                       ^^^
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>

warning: 2 warnings emitted

 */
fn main() {
    let mut buffer = String::new();

    stdin().read_line(&mut buffer).expect("failed read line");
    buffer.pop();
    let num = buffer.parse::<f64>().expect("not valid number");

    let info = if let 0.0 = num {
        "zero"
    } else {
        if num > 0.0 { "positive integer" } else { "negative real" }
    };
    println!("{}", info)
}
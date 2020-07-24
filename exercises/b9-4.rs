use core::fmt;
use std::fmt::Formatter;
use std::ptr::write;

fn main() {
    let mut z = Complex::new();
    z.set(3., -2.);
    z= z.add(z.clone());
    z.show();
}

#[derive(Clone)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new() -> Complex {
        Complex {
            real: 0 as f64,
            imag: 0 as f64,
        }
    }
    //  help: convert the identifier to snake case: `get_real`
    fn getReal(&self) -> f64 {
        self.real
    }
    fn getImag(&self) -> f64 {
        self.imag
    }
    fn set(&mut self, real: f64, imag: f64) {
        self.real = real;
        self.imag = imag;
    }
    fn module(&self) -> f64 {
        (self.imag.powi(2) + self.real.powi(2)).sqrt()
    }
    fn add(&self, operand: Complex ) -> Complex {
        Complex{
            real: self.real+operand.real,
            imag: self.imag+operand.imag,
        }
    }
    fn show(&self) {
        println!("{}", self)
    }
}

impl fmt::Display  for Complex{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.imag < 0.0 {
            write!(f,"{}-j{}",self.real, -self.imag)
        } else {
            write!(f,"{}+j{}",self.real, self.imag)
        }
    }
}
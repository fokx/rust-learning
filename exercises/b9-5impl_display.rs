use core::fmt;
use std::fmt::Formatter;
use std::ptr::write;
use std::cmp::{ min};

fn main() {
    let mut z = Fraction::new(8, -24);
    z.show();
    println!("{}", z.real())
}

#[derive(Clone)]
struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    fn new(mut numerator: i64, mut denominator: i64) -> Fraction {
        if denominator == 0 {
            panic!("denominator cannot be zero")
        } else if denominator < 0 {
            denominator = -denominator;
            numerator = -numerator;
        }
        for i in (2..=min(denominator.abs(),numerator.abs())).rev(){
            if denominator % i == 0 && numerator % i == 0 {
                denominator /= i;
                numerator /= i;
                break // break because we start from large value
            }
        }
        assert!(denominator.is_positive());
        Fraction {
            numerator: numerator,
            denominator: denominator,
        }
    }

    fn set(&mut self, numerator: i64, denominator: i64) {
        self.numerator = numerator;
        self.denominator = denominator;
    }
    fn real(&self) -> f64 {
        // will not work if use:
        // (self.numerator / self.denominator) as f64
        self.numerator as f64 / self.denominator as f64
    }
    fn show(&self) {
        println!("{}", self)
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
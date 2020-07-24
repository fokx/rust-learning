use std::thread::sleep;
use core::ops;
use std::ops::Add;

fn main() {
    let a = Vector3d{x:1.,y:2.,z:3.};
    let b = Vector3d{x:3.,y:4.,z:5.};
    let c = Vector3d::new();
    a.show();
    b.show();
    c.show();
    let c = a+b;
    c.show()
}

struct Vector3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3d {
    fn new() -> Vector3d {
        Vector3d{
            x: 0.0,
            y: 0.0,
            z: 0.0

        }
    }
    fn set(&mut self,x:f64,y:f64,z:f64){
        self.x = x;
        self.y = y;
        self.z = z;

    }
    fn show(&self) {
        println!("({}, {}, {})", self.x, self.y, self.z)
    }

}

impl ops::Add<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Self::Output {
        Vector3d{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}
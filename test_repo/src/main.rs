#![allow(dead_code)]

#[derive(Debug)]
struct Vec3(f64, f64, f64);

impl Vec3 {

    fn norm(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    fn normalize(&self) -> Vec3 {
        Vec3(
            self.0 / self.norm(),
            self.1 / self.norm(),
            self.2 / self.norm(),
        )
    }

}

fn test_vec3() {

    let v1 = Vec3(10.0, 10.0, 10.0);

    let v1_norm = v1.norm();

    println!("v1 is {:?} it's norm is {}", v1, v1_norm);

    let v2 = v1.normalize();

    println!("the normalized vector of v1 is {:?}", v2);

}

fn foo(x: i32) -> i32 {
    x + 1
}

fn main() {

    let n: i32 = 5;
    let y: i32 = foo(n);

    println!("{} {}", y, n);

}

use crate::la::vector::Vector;
use crate::la::ops;

pub trait Activation {
    fn f(&self, v: Vector) -> Vector;
}

pub struct Sigmoid {}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

impl Activation for Sigmoid {

    fn f(&self, v: Vector) -> Vector {
        ops::f(v, sigmoid)
    }

}
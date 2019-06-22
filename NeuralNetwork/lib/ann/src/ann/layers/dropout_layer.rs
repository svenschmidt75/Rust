use crate::ann::activation::Activation;
use crate::ann::layers::layer::Layer;
use linear_algebra::matrix::Matrix2D;
use linear_algebra::vector::Vector;

pub struct DropoutLayer {
    p: f64
}

impl DropoutLayer {}

impl Layer for DropoutLayer {
    fn initialize(&mut self, prev_layer: &Layer) {
        unimplemented!()
    }

    fn feedforward(&self, a: &Vector) -> (Vector, Vector) {
        unimplemented!()
    }

    fn nactivations(&self) -> usize {
        unimplemented!()
    }

    fn get_weights(&self) -> &Matrix2D {
        unimplemented!()
    }

    fn set_weights(&mut self, weights: Matrix2D) {
        unimplemented!()
    }

    fn get_weights_mut(&mut self) -> &mut Matrix2D {
        unimplemented!()
    }

    fn get_momentum_weights(&self) -> &Matrix2D {
        unimplemented!()
    }

    fn set_momentum_weights(&mut self, momentum_weights: Matrix2D) {
        unimplemented!()
    }

    fn get_biases(&self) -> &Vector {
        unimplemented!()
    }

    fn get_biases_mut(&mut self) -> &mut Vector {
        unimplemented!()
    }

    fn set_biases(&mut self, biases: Vector) {
        unimplemented!()
    }

    fn get_momentum_biases(&self) -> &Vector {
        unimplemented!()
    }

    fn set_momentum_biases(&mut self, momentum_biases: Vector) {
        unimplemented!()
    }

    fn get_activation(&self) -> &Activation {
        unimplemented!()
    }

    fn weights_squared_sum(&self) -> f64 {
        unimplemented!()
    }

    fn print_summary(&self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}

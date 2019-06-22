use crate::ann::activation::Activation;
use crate::ann::layers::layer::Layer;
use linear_algebra::matrix::Matrix2D;
use linear_algebra::vector::Vector;

pub struct InputLayer {
    nactivations: usize,
}

impl InputLayer {
    pub fn new(nactivations: usize) -> InputLayer {
        InputLayer { nactivations }
    }
}

impl Layer for InputLayer {
    fn initialize(&mut self, _prev_layer: &Layer) {}

    fn feedforward(&self, _a: &Vector) -> (Vector, Vector) {
        unreachable!()
    }

    fn nactivations(&self) -> usize {
        self.nactivations
    }

    fn get_weights(&self) -> &Matrix2D {
        unreachable!()
    }

    fn set_weights(&mut self, _weights: Matrix2D) {
        unreachable!()
    }

    fn get_weights_mut(&mut self) -> &mut Matrix2D {
        unreachable!()
    }

    fn get_momentum_weights(&self) -> &Matrix2D {
        unreachable!()
    }

    fn set_momentum_weights(&mut self, _momentum_weights: Matrix2D) {
        unreachable!()
    }

    fn get_biases(&self) -> &Vector {
        unreachable!()
    }

    fn get_biases_mut(&mut self) -> &mut Vector {
        unreachable!()
    }

    fn set_biases(&mut self, _biases: Vector) {
        unreachable!()
    }

    fn get_momentum_biases(&self) -> &Vector {
        unreachable!()
    }

    fn set_momentum_biases(&mut self, _momentum_biases: Vector) {
        unreachable!()
    }

    fn get_activation(&self) -> &Activation {
        unreachable!()
    }

    fn weights_squared_sum(&self) -> f64 {
        unreachable!()
    }

    fn print_summary(&self) {
        unreachable!()
    }
}

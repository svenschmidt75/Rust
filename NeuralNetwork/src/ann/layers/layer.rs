use rand::Rng;

use crate::ann::activation::Activation;
use crate::la::matrix::Matrix2D;
use crate::la::ops;
use crate::la::vector::Vector;

pub trait Layer {
    fn initialize(&mut self);

    //    fn on_start_new_epoch();
    fn feedforward(&self, a: &Vector) -> (Vector, Vector);

    fn nactivations(&self) -> usize;

    fn get_weights(&self) -> &Matrix2D;

    fn get_weights_mut(&mut self) -> &mut Matrix2D;

    fn get_biases_mut(&mut self) -> &mut Vector;

    fn get_activation(&self) -> &Activation;
}

pub struct FCLayer {
    weights: Matrix2D,
    biases: Vector,
    activation: Box<dyn Activation>,
}

impl FCLayer {
    pub fn new(weights: Matrix2D, biases: Vector, activation: Box<dyn Activation>) -> FCLayer {
        assert_eq!(weights.nrows(), biases.dim());
        FCLayer { weights, biases, activation }
    }

    fn get_weight(&self, i: usize, j: usize) -> f64 {
        // i: index of activation in layer l
        // j: index of activation in layer l-1
        self.weights[(i, j)]
    }
}

impl Layer for FCLayer {
    fn initialize(&mut self) {
        let mut rng = rand::thread_rng();
        for row in 0..self.weights.nrows() {
            for col in 0..self.weights.ncols() {
                let value: f64 = rng.gen();
                self.weights[(row, col)] = value / 100.0;
            }
        }

        for idx in 0..self.biases.dim() {
            let value: f64 = rng.gen();
            self.biases[idx] = value / 100.0;
        }
    }

    fn feedforward(&self, input: &Vector) -> (Vector, Vector) {
        // SS: number of activations in this layer: self.weights.nrows()
        let output = ops::ax(&self.weights, input);

        // SS: alternatively, add another column to weights with the biases.
        // Add another row with all 0s, except for the bias column where we put 1.
        let z = &output + &self.biases;
        let a = self.activation.f(&z);
        (a, z)
    }

    fn nactivations(&self) -> usize {
        self.biases.dim()
    }

    fn get_weights(&self) -> &Matrix2D {
        &self.weights
    }

    fn get_weights_mut(&mut self) -> &mut Matrix2D {
        &mut self.weights
    }

    fn get_biases_mut(&mut self) -> &mut Vector {
        &mut self.biases
    }

    fn get_activation(&self) -> &Activation {
        &*self.activation
    }
}

pub struct InputLayer {
    nactivations: usize,
}

impl InputLayer {
    pub fn new(nactivations: usize) -> InputLayer {
        InputLayer { nactivations }
    }
}

impl Layer for InputLayer {
    fn initialize(&mut self) {}

    fn feedforward(&self, _a: &Vector) -> (Vector, Vector) {
        unreachable!()
    }

    fn nactivations(&self) -> usize {
        self.nactivations
    }

    fn get_weights(&self) -> &Matrix2D {
        unreachable!()
    }

    fn get_weights_mut(&mut self) -> &mut Matrix2D {
        unreachable!()
    }

    fn get_biases_mut(&mut self) -> &mut Vector {
        unreachable!()
    }

    fn get_activation(&self) -> &Activation {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::ann::activation::Id;

    use super::*;

    #[test]
    fn test_initialize() {
        // Arrange
        let weights1 = Matrix2D::new_from_data(2, 2, vec![0.1, 0.1, 0.1, 0.1]);
        let biases1: Vector = vec![0.2, 0.2].into();
        let mut layer = FCLayer::new(weights1.clone(), biases1.clone(), Box::new(Id {}));

        // Act
        layer.initialize();

        // Assert
        assert!(layer.get_weights()[(0, 0)] <= 1.0 / 100.0);
    }
}

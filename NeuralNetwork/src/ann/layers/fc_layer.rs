use crate::ann::activation::Activation;
use crate::ann::layers::layer::Layer;
use crate::la::matrix::Matrix;
use crate::la::matrix::Matrix2D;
use crate::la::ops;
use crate::la::vector::Vector;

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
        self.weights.get(i, j)
    }
}

impl Layer for FCLayer {
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

    fn get_weights(&self) -> &Matrix {
        &self.weights
    }

    fn get_activation(&self) -> &Activation {
        &*self.activation
    }
}

#[cfg(test)]
mod tests {
    //    use super::*;

    #[test]
    fn test_index() {}
}

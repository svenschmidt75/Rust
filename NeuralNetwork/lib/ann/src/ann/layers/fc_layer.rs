use rand::Rng;

use crate::ann::activation::Activation;
use crate::ann::layers::layer::Layer;
use linear_algebra::matrix::Matrix2D;
use linear_algebra::ops;
use linear_algebra::vector::Vector;
use crate::ann::minibatch::Minibatch;
use crate::ann::cost_function::CostFunction;

pub struct FCLayer {
    weights: Matrix2D,
    momentum_weights: Matrix2D,
    biases: Vector,
    momentum_biases: Vector,
    pub nneurons: usize,
    activation: Box<dyn Activation>,
}

impl FCLayer {
    pub fn new(nneurons: usize, activation: Box<dyn Activation>) -> FCLayer {
        FCLayer {
            weights: Matrix2D::new(0, 0),
            momentum_weights: Matrix2D::new(0, 0),
            biases: Vector::new(0),
            momentum_biases: Vector::new(0),
            activation,
            nneurons,
        }
    }

    fn initialize_parameters(&mut self, fan_in: usize) {
        // SS: modified Xavier initialization
        //Stanford Neural networks, Lecture 6, https://www.youtube.com/watch?v=wEoyxE0GP2M&list=PL3FW7Lu3i5JvHM8ljYj-zLfQRF3EO8sYv&index=6
        let mut rng = rand::thread_rng();
        let std_dev = 1.0 / (fan_in as f64 / 2.0);
        for row in 0..self.weights.nrows() {
            for col in 0..self.weights.ncols() {
                let value: f64 = rng.gen();
                self.weights[(row, col)] = value * std_dev;
            }
        }
        for idx in 0..self.biases.dim() {
            self.biases[idx] = 0.0;
        }
    }

    pub(crate) fn initialize(&mut self, prev_layer: &Layer) {
        let n = prev_layer.nactivations();
        self.weights = Matrix2D::new(self.nneurons, n);
        self.momentum_weights = Matrix2D::new(self.nneurons, n);
        self.biases = Vector::new(self.nneurons);
        self.momentum_biases = Vector::new(self.nneurons);
        self.initialize_parameters(n);
    }

    pub(crate) fn feedforward(&self, prev_a: &Vector) -> (Vector, Vector) {
        // SS: number of activations in this layer: self.weights.nrows()
        let output = ops::ax(&self.weights, prev_a);

        // SS: alternatively, add another column to weights with the biases.
        // Add another row with all 0s, except for the bias column where we put 1.
        let z = &output + &self.biases;
        let a = self.activation.f(&z);
        (a, z)
    }

    pub fn calculate_outputlayer_error(&self, a: &Vector, z: &Vector, cost_function: &CostFunction, y: &Vector) -> Vector {
        // SS: calculate delta_{L}, the error in the output layer
        let sigma = self.get_activation();
        let output_error = cost_function.output_error(a, z, y, sigma);
        output_error
    }

    fn nactivations(&self) -> usize {
        self.nneurons
    }

    fn get_weights(&self) -> &Matrix2D {
        &self.weights
    }

    fn set_weights(&mut self, weights: Matrix2D) {
        self.weights = weights;
    }

    fn get_weights_mut(&mut self) -> &mut Matrix2D {
        &mut self.weights
    }

    fn get_momentum_weights(&self) -> &Matrix2D {
        &self.momentum_weights
    }

    fn set_momentum_weights(&mut self, momentum_weights: Matrix2D) {
        self.momentum_weights = momentum_weights;
    }

    fn get_biases(&self) -> &Vector {
        &self.biases
    }

    fn get_biases_mut(&mut self) -> &mut Vector {
        &mut self.biases
    }

    fn set_biases(&mut self, biases: Vector) {
        self.biases = biases;
    }

    fn get_momentum_biases(&self) -> &Vector {
        &self.momentum_biases
    }

    fn set_momentum_biases(&mut self, momentum_biases: Vector) {
        self.momentum_biases = momentum_biases;
    }

    fn get_activation(&self) -> &Activation {
        &*self.activation
    }

    fn weights_squared_sum(&self) -> f64 {
        let mut w2 = 0.0;
        for col in 0..self.weights.ncols() {
            for row in 0..self.weights.nrows() {
                let w = self.weights[(row, col)];
                w2 += w * w;
            }
        }
        w2
    }

    pub(crate) fn print_summary(&self) {
        let nparams = self.weights.ncols() * self.weights.nrows() + self.biases.dim();
        println!("{:15} | {:15} | {:15}", "dense", self.nneurons, nparams);
    }
}

#[cfg(test)]
mod tests {
    use crate::ann::activation::Id;

    use super::*;

    #[test]
    fn test_initialize() {
        // Arrange
        let layer1 = FCLayer::new(2, Box::new(Id {}));
        let mut layer2 = FCLayer::new(2, Box::new(Id {}));

        // Act
        layer2.initialize(&layer1);

        // Assert
        assert!(layer2.get_weights()[(0, 0)] <= 1.0 / 100.0);
    }
}

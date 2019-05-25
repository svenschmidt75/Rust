use rand::Rng;

use crate::ann::activation::Activation;
use linear_algebra::matrix::Matrix2D;
use linear_algebra::ops;
use linear_algebra::vector::Vector;

pub trait Layer {
    fn initialize(&mut self, prev_layer: &Layer);

    //    fn on_start_new_epoch();
    fn feedforward(&self, a: &Vector) -> (Vector, Vector);

    fn nactivations(&self) -> usize;

    fn get_weights(&self) -> &Matrix2D;

    fn set_weights(&mut self, weights: Matrix2D);

    fn get_weights_mut(&mut self) -> &mut Matrix2D;

    fn get_momentum_weights(&self) -> &Matrix2D;

    fn set_momentum_weights(&mut self, momentum_weights: Matrix2D);

    fn get_biases(&self) -> &Vector;

    fn get_biases_mut(&mut self) -> &mut Vector;

    fn set_biases(&mut self, biases: Vector);

    fn get_momentum_biases(&self) -> &Vector;

    fn set_momentum_biases(&mut self, momentum_biases: Vector);

    fn get_activation(&self) -> &Activation;

    fn weights_squared_sum(&self) -> f64;

    fn print_summary(&self);
}

pub struct FCLayer {
    weights: Matrix2D,
    momentum_weights: Matrix2D,
    biases: Vector,
    momentum_biases: Vector,
    nneurons: usize,
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

    fn get_weight(&self, i: usize, j: usize) -> f64 {
        // i: index of activation in layer l
        // j: index of activation in layer l-1
        self.weights[(i, j)]
    }

    fn initialize_parameters(&mut self, fan_in: usize, fan_out: usize) {
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
            let value: f64 = rng.gen();
            self.biases[idx] = 0.0;
        }
    }
}

impl Layer for FCLayer {
    fn initialize(&mut self, prev_layer: &Layer) {
        let n = prev_layer.nactivations();
        self.weights = Matrix2D::new(self.nneurons, n);
        self.momentum_weights = Matrix2D::new(self.nneurons, n);
        self.biases = Vector::new(self.nneurons);
        self.momentum_biases = Vector::new(self.nneurons);
        self.initialize_parameters(n, self.nneurons);
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
        self.nneurons
    }

    fn get_weights(&self) -> &Matrix2D {
        &self.weights
    }

    fn get_weights_mut(&mut self) -> &mut Matrix2D {
        &mut self.weights
    }

    fn set_weights(&mut self, weights: Matrix2D) {
        self.weights = weights;
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

    fn print_summary(&self) {
        let nparams = self.weights.ncols() * self.weights.nrows() + self.biases.dim();
        println!("{:15} | {:15} | {:15}", "dense", self.nneurons, nparams);
    }

    fn get_momentum_weights(&self) -> &Matrix2D {
        &self.momentum_weights
    }

    fn set_momentum_weights(&mut self, momentum_weights: Matrix2D) {
        self.momentum_weights = momentum_weights;
    }

    fn get_momentum_biases(&self) -> &Vector {
        &self.momentum_biases
    }

    fn set_momentum_biases(&mut self, momentum_biases: Vector) {
        self.momentum_biases = momentum_biases;
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
    fn initialize(&mut self, prev_layer: &Layer) {}

    fn feedforward(&self, _a: &Vector) -> (Vector, Vector) {
        unreachable!()
    }

    fn nactivations(&self) -> usize {
        self.nactivations
    }

    fn get_weights(&self) -> &Matrix2D {
        unreachable!()
    }

    fn set_weights(&mut self, weights: Matrix2D) {
        unreachable!()
    }

    fn get_weights_mut(&mut self) -> &mut Matrix2D {
        unreachable!()
    }

    fn get_momentum_weights(&self) -> &Matrix2D {
        unreachable!()
    }

    fn set_momentum_weights(&mut self, momentum_weights: Matrix2D) {
        unreachable!()
    }

    fn get_biases(&self) -> &Vector {
        unreachable!()
    }

    fn get_biases_mut(&mut self) -> &mut Vector {
        unreachable!()
    }

    fn set_biases(&mut self, biases: Vector) {
        unreachable!()
    }

    fn get_momentum_biases(&self) -> &Vector {
        unreachable!()
    }

    fn set_momentum_biases(&mut self, momentum_biases: Vector) {
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

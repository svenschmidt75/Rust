use rand::Rng;

use crate::ann::layers::layer::Layer;
use crate::ann::minibatch::Minibatch;
use linear_algebra::matrix::Matrix2D;
use linear_algebra::ops;
use linear_algebra::vector::Vector;

pub struct FCLayer {
    weights: Matrix2D,
    momentum_weights: Matrix2D,
    biases: Vector,
    momentum_biases: Vector,
    nneurons: usize,
}

impl FCLayer {
    pub fn new(input_size: usize) -> FCLayer {
        FCLayer {
            weights: Matrix2D::new(0, 0),
            momentum_weights: Matrix2D::new(0, 0),
            biases: Vector::new(0),
            momentum_biases: Vector::new(0),
            nneurons: input_size,
        }
    }

    pub fn number_of_neurons(&self) -> usize {
        self.nneurons
    }

    pub(crate) fn initialize(&mut self, prev_layer: &Layer) {
        let ncols = prev_layer.number_of_neurons();
        let nrows = self.number_of_neurons();

        self.weights = Matrix2D::new(nrows, ncols);
        self.biases = Vector::new(nrows);

        self.momentum_weights = Matrix2D::new(nrows, ncols);
        self.momentum_biases = Vector::new(nrows);

        self.initialize_parameters(ncols);
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

    pub(crate) fn feedforward(&self, prev_a: &Vector) -> Vector {
        // SS: number of activations in this layer: self.weights.nrows()
        let output = self.weights.ax(prev_a);

        // SS: alternatively, add another column to weights with the biases.
        // Add another row with all 0s, except for the bias column where we put 1.
        let z = &output + &self.biases;
        z
    }

    pub fn backprop(&self, layer_index: usize, mb: &mut Minibatch) {
        assert!(layer_index > 0);

        // SS: calculate dz^{l}/da^{l-1}
        let weights = &self.weights.transpose();
        let delta_next = &mb.error[layer_index + 1];
        let delta = weights.ax(delta_next);
        mb.error[layer_index] = delta;
    }

    pub(crate) fn get_weights(&self) -> &Matrix2D {
        &self.weights
    }

    pub(crate) fn set_weights(&mut self, weights: Matrix2D) {
        self.weights = weights;
    }

    fn get_momentum_weights(&self) -> &Matrix2D {
        &self.momentum_weights
    }

    fn set_momentum_weights(&mut self, momentum_weights: Matrix2D) {
        self.momentum_weights = momentum_weights;
    }

    pub(crate) fn get_biases(&self) -> &Vector {
        &self.biases
    }

    pub(crate) fn set_biases(&mut self, biases: Vector) {
        self.biases = biases;
    }

    fn get_momentum_biases(&self) -> &Vector {
        &self.momentum_biases
    }

    fn set_momentum_biases(&mut self, momentum_biases: Vector) {
        self.momentum_biases = momentum_biases;
    }

    pub(crate) fn weights_squared_sum(&self) -> f64 {
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

    pub fn update_network(&mut self, layer_index: usize, mbs: &[Minibatch], eta: f64, rho: f64, lambda: f64) {
        // calc. derivatives from all mini batches
        let (dw, db) = self.calculate_derivatives(layer_index, mbs, lambda);

        // calc. momentum
        self.apply_momentum(eta, rho, dw, db);

        // update weights and biases
        self.update_parameters();
    }

    pub fn calculate_derivatives(&self, layer_index: usize, mbs: &[Minibatch], lambda: f64) -> (Matrix2D, Vector) {
        let mut dw = Matrix2D::new(self.weights.nrows(), self.weights.ncols());
        let mut db = Vector::new(self.weights.nrows());

        for mb in mbs {
            let delta_next = &mb.error[layer_index + 1];
            let a_j = &mb.output[layer_index - 1];

            let dw_ij = ops::outer_product(delta_next, a_j);
            dw += &dw_ij;

            let db_i = delta_next;
            db += &db_i;
        }

        let w = self.get_weights();
        dw = &(&dw + &(lambda * w)) / (mbs.len() as f64);

        db /= mbs.len();

        (dw, db)
    }

    fn apply_momentum(&mut self, eta: f64, rho: f64, dw: Matrix2D, db: Vector) {
        let momentum_weights = self.get_momentum_weights();
        let updates_momentum_weights = &(rho * momentum_weights) - &(eta * &dw);
        self.set_momentum_weights(updates_momentum_weights);

        let momentum_biases = self.get_momentum_biases();
        let updated_momentum_biases = &(rho * momentum_biases) - &(eta * &db);
        self.set_momentum_biases(updated_momentum_biases);
    }

    fn update_parameters(&mut self) {
        {
            let weights = self.get_weights();
            let momentum_weights = self.get_momentum_weights();
            let updated_weights = weights + momentum_weights;
            self.set_weights(updated_weights);
        }
        {
            let biases = self.get_biases();
            let momentum_biases = self.get_momentum_biases();
            let updated_biases = biases + momentum_biases;
            self.set_biases(updated_biases);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ann::layers::layer::Layer::FullyConnected;

    #[test]
    fn test_initialize() {
        // Arrange
        let layer1 = FullyConnected(FCLayer::new(2));
        let mut layer2 = FCLayer::new(2);

        // Act
        layer2.initialize(&layer1);

        // Assert
        assert!(layer2.get_weights()[(0, 0)] <= 1.0);
    }
}

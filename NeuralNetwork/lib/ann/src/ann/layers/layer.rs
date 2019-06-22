#![allow(dead_code)]

use crate::ann::activation::Activation;
use linear_algebra::matrix::Matrix2D;
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

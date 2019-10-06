use crate::ann::layers::layer::Layer;
use crate::ann::minibatch::Minibatch;
use linear_algebra::vector::Vector;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use std::cell::RefCell;

pub struct BatchNormalizeLayer {
    nneurons: usize,
}

impl BatchNormalizeLayer {
    pub fn new(nneurons: usize) -> BatchNormalizeLayer {
        BatchNormalizeLayer {
            nneurons,
        }
    }

    pub(crate) fn initialize(&mut self, prev_layer: &Layer) {
        assert_eq!(self.nneurons, prev_layer.number_of_neurons());
    }

    pub (crate) fn number_of_neurons(&self) -> usize {
        self.nneurons
    }

    pub(crate) fn next_minibatch(&mut self, mbs: &[Minibatch]) {
    }

    pub(crate) fn backprop(&self, layer_index: usize, mb: &mut Minibatch) {
        assert!(layer_index > 0);
    }

    pub(crate) fn print_summary(&self) {
        println!("{:15} | {:15} | {:15}", "BN", "", "");
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use linear_algebra::vector::Vector;

    use super::*;

    #[test]
    fn test_backprop_gradient() {
        // Arrange
//        let dropout_probability = 0.75;
//        let mut layer = DropoutLayer::new(2, dropout_probability, Box::new(TestUniformDistributionSampler { p: dropout_probability }));
//        let mut mb = Minibatch::new(vec![2, 2, 2]);
//
//        let z0 = 0.765;
//        let z1 = 0.134;
//
//        mb.output[0] = Vector::from(vec![z0, z1]);
//
//        // calculate a from z
//        layer.next_training_sample();
//        mb.output[1] = layer.feedforward(&mb.output[0]);
//        let a0 = mb.output[1][0];
//        let a1 = mb.output[1][1];
//
//        // C(a0, a1) = -3 * sin(a0) + 5 * cos(a1)
//
//        // set dC/da
//        let dCda0 = -3.0 * a0.cos();
//        let dCda1 = -5.0 * a1.sin();
//        mb.error[2] = Vector::from(vec![dCda0, dCda1]);
//
//        // Act
//        layer.backprop(1, &mut mb);
//
//        // Assert
//        // dCdz0 = dCda0 * da0dz0 + dCda1 * da1dz0
//        let dCdz0 = dCda0 * 1.0 / dropout_probability;
//        let dCdz1 = dCda1 * 1.0 / dropout_probability;
//        assert_approx_eq!(dCdz0, mb.error[1][0], 1E-12);
//        assert_approx_eq!(dCdz1, mb.error[1][1], 1E-12);
    }

}

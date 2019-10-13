use crate::ann::layers::layer::Layer;
use crate::ann::minibatch::Minibatch;
use linear_algebra::ops;
use linear_algebra::vector::Vector;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use std::cell::RefCell;

pub struct BatchNormalizeLayer {
    nneurons: usize,
    mean: Vector,
    variance: Vector,
    stddev: Vector,
    one_over_stddev: Vector,
    gamma: Vector,
    beta: Vector,
}

impl BatchNormalizeLayer {
    pub fn new(nneurons: usize) -> BatchNormalizeLayer {
        BatchNormalizeLayer {
            nneurons,
            mean: Vector::new(0),
            variance: Vector::new(0),
            stddev: Vector::new(0),
            one_over_stddev: Vector::new(0),
            gamma: Vector::new(0),
            beta: Vector::new(0),
        }
    }

    pub(crate) fn initialize(&mut self, prev_layer: &Layer) {
        assert_eq!(self.nneurons, prev_layer.number_of_neurons());
        self.gamma  = Vector::new(self.nneurons);
        self.beta = Vector::new(self.nneurons);
    }

    pub(crate) fn number_of_neurons(&self) -> usize {
        self.nneurons
    }

    pub(crate) fn next_minibatch(&mut self, mbs: &[Minibatch], layer_index: usize) {
        // SS: calculate mean and variance across minibatch
        self.mean = BatchNormalizeLayer::mean(mbs, layer_index);
        self.variance = BatchNormalizeLayer::variance(mbs, layer_index, &self.mean);
        self.stddev = BatchNormalizeLayer::stddev(&self.variance);
        self.one_over_stddev = ops::f(&self.stddev, &|x| 1.0 / x);
    }

    fn variance(mbs: &[Minibatch], layer_index: usize, means: &Vector) -> Vector {
        assert!(mbs.len() > 0);
        let msize = mbs[0].output[layer_index].dim();
        let mut variance = mbs.iter().fold(Vector::new(msize), |mut accum, mb| {
            let output = &mb.output[layer_index];
            let tmp = output - &means;
            let tmp2 = ops::hadamard(&tmp, &tmp);
            accum += &tmp2;
            accum
        });
        variance /= mbs.len();
        variance
    }

    fn stddev(variance: &Vector) -> Vector {
        ops::f(variance, &f64::sqrt)
    }

    fn mean(mbs: &[Minibatch], layer_index: usize) -> Vector {
        assert!(mbs.len() > 0);
        let msize = mbs[0].output[layer_index].dim();
        let mut means = mbs.iter().fold(Vector::new(msize), |mut accum, mb| {
            let output = &mb.output[layer_index];
            accum += output;
            accum
        });
        means /= mbs.len();
        means
    }

    pub(crate) fn feedforward(&self, z: &Vector) -> Vector {
        let x_hat = self.x_hat(z);
        let y = &ops::hadamard(&x_hat, &self.gamma) + &self.beta;
        return y;
    }

    fn x_hat(&self, z: &Vector) -> Vector {
        assert_eq!(z.dim(), self.mean.dim());
        let tmp1 = z - &self.mean;
        let x_hat = ops::hadamard(&tmp1, &self.one_over_stddev);
        x_hat
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

    use super::*;

    #[test]
    fn test_means() {
        // Arrange
        let mut mb1 = Minibatch::new(vec![2, 3, 2]);
        let mut mb2 = Minibatch::new(vec![2, 3, 2]);
        let mut mb3 = Minibatch::new(vec![2, 3, 2]);
        let mut mb4 = Minibatch::new(vec![2, 3, 2]);
        let mut mbs = [mb1, mb2, mb3, mb4];

        mbs[0].output[1][0] = 7.0;
        mbs[0].output[1][1] = 9.0;
        mbs[0].output[1][2] = 10.0;

        mbs[1].output[1][0] = 3.0;
        mbs[1].output[1][1] = -2.0;
        mbs[1].output[1][2] = 6.0;

        mbs[2].output[1][0] = 4.0;
        mbs[2].output[1][1] = 3.0;
        mbs[2].output[1][2] = 2.0;

        mbs[3].output[1][0] = 12.0;
        mbs[3].output[1][1] = 13.0;
        mbs[3].output[1][2] = 6.0;

        // Act
        let means = BatchNormalizeLayer::mean(&mbs, 1);

        // Assert
        assert_eq!(means.dim(), 3);
        assert_approx_eq!(means[0], 6.5);
        assert_approx_eq!(means[1], 5.75);
        assert_approx_eq!(means[2], 6.0);
    }

    #[test]
    fn test_variance() {
        // Arrange
        let mut mb1 = Minibatch::new(vec![2, 3, 2]);
        let mut mb2 = Minibatch::new(vec![2, 3, 2]);
        let mut mb3 = Minibatch::new(vec![2, 3, 2]);
        let mut mb4 = Minibatch::new(vec![2, 3, 2]);
        let mut mbs = [mb1, mb2, mb3, mb4];

        mbs[0].output[1][0] = 7.0;
        mbs[0].output[1][1] = 9.0;
        mbs[0].output[1][2] = 10.0;

        mbs[1].output[1][0] = 3.0;
        mbs[1].output[1][1] = -2.0;
        mbs[1].output[1][2] = 6.0;

        mbs[2].output[1][0] = 4.0;
        mbs[2].output[1][1] = 3.0;
        mbs[2].output[1][2] = 2.0;

        mbs[3].output[1][0] = 12.0;
        mbs[3].output[1][1] = 13.0;
        mbs[3].output[1][2] = 6.0;

        let means = BatchNormalizeLayer::mean(&mbs, 1);

        // Act
        let variance = BatchNormalizeLayer::variance(&mbs, 1, &means);

        // Assert
        assert_eq!(variance.dim(), 3);
        assert_approx_eq!(variance[0], 12.25);
        assert_approx_eq!(variance[1], 32.6875);
        assert_approx_eq!(variance[2], 8.0);
    }

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

use crate::ann::layers::layer::Layer;
use crate::ann::minibatch::Minibatch;
use linear_algebra::vector::Vector;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;
use std::cell::RefCell;

pub struct DropoutLayer {
    nneurons: usize,
    p: f64,
    sampler: Box<dyn UniformDistributionSampler>,
    probability_vector: Vector,
}

impl DropoutLayer {
    pub fn new(nneurons: usize, p: f64, sampler: Box<dyn UniformDistributionSampler>) -> DropoutLayer {
        DropoutLayer {
            nneurons,
            p,
            sampler,
            probability_vector: Vector::new(nneurons),
        }
    }

    pub(crate) fn initialize(&mut self, prev_layer: &Layer) {
        assert_eq!(self.nneurons, prev_layer.number_of_neurons());
    }

    pub fn number_of_neurons(&self) -> usize {
        self.nneurons
    }

    pub(crate) fn new_feedforward(&mut self) {
        for idx in 0..self.probability_vector.dim() {
            let factor = {
                let v = self.sampler.sample();
                if v < self.p {
                    0.0
                } else {
                    // SS: inverted dropout
                    1.0 / self.p
                }
            };
            self.probability_vector[idx] = factor;
        }
    }

    pub(crate) fn feedforward(&mut self, layer_index: usize, mbs: &mut [Minibatch]) {
        assert!(layer_index > 0);
        for mb in mbs {
            self.new_feedforward();
            let input = &mb.output[layer_index - 1];
            let a = input.hadamard(&self.probability_vector);
            mb.output[layer_index] = a;
        }
    }

    pub fn backprop(&self, layer_index: usize, mb: &mut Minibatch) {
        assert!(layer_index > 0);
        let delta_next = &mb.error[layer_index + 1];
        let delta = delta_next.hadamard(&self.probability_vector);
        mb.error[layer_index] = delta;
    }

    pub(crate) fn print_summary(&self) {
        println!("{:15} | {:>15} | {:15}", "dropout", format!("p={}", self.p), "");
    }
}

pub trait UniformDistributionSampler {
    fn sample(&self) -> f64;
}

pub struct ProdUniformDistributionSampler {
    rng: RefCell<ThreadRng>,
}

impl ProdUniformDistributionSampler {
    pub fn new() -> ProdUniformDistributionSampler {
        ProdUniformDistributionSampler {
            rng: RefCell::new(rand::thread_rng()),
        }
    }
}

impl UniformDistributionSampler for ProdUniformDistributionSampler {
    fn sample(&self) -> f64 {
        let distribution = Uniform::new(0.0, 1.0);
        let r: &mut ThreadRng = &mut self.rng.borrow_mut();
        distribution.sample(r)
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use linear_algebra::vector::Vector;

    use super::*;

    struct TestUniformDistributionSampler {
        p: f64,
    }

    impl UniformDistributionSampler for TestUniformDistributionSampler {
        fn sample(&self) -> f64 {
            self.p
        }
    }

    #[test]
    fn test_backprop_gradient() {
        // Arrange
        let dropout_probability = 0.75;
        let mut layer = DropoutLayer::new(2, dropout_probability, Box::new(TestUniformDistributionSampler { p: dropout_probability }));
        let mut mb = Minibatch::new(vec![2, 2, 2]);
        let mut mbs = [mb];

        let z0 = 0.765;
        let z1 = 0.134;

        mbs[0].output[0] = Vector::from(vec![z0, z1]);

        // calculate a from z
        layer.new_feedforward();
        layer.feedforward(1, &mut mbs[..]);
        let a0 = mbs[0].output[1][0];
        let a1 = mbs[0].output[1][1];

        // C(a0, a1) = -3 * sin(a0) + 5 * cos(a1)

        // set dC/da
        let dCda0 = -3.0 * a0.cos();
        let dCda1 = -5.0 * a1.sin();
        mbs[0].error[2] = Vector::from(vec![dCda0, dCda1]);

        // Act
        layer.backprop(1, &mut mbs[0]);

        // Assert
        // dCdz0 = dCda0 * da0dz0 + dCda1 * da1dz0
        let dCdz0 = dCda0 * 1.0 / dropout_probability;
        let dCdz1 = dCda1 * 1.0 / dropout_probability;
        assert_approx_eq!(dCdz0, mbs[0].error[1][0], 1E-12);
        assert_approx_eq!(dCdz1, mbs[0].error[1][1], 1E-12);
    }
}

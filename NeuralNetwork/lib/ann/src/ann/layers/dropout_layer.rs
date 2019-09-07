use crate::ann::layers::layer::Layer;
use crate::ann::minibatch::Minibatch;
use linear_algebra::vector::Vector;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::ThreadRng;

pub struct DropoutLayer {
    nneurons: usize,
    p: f64,
    probability_vector: Vector,
    rng: ThreadRng,
}

impl DropoutLayer {
    pub fn new(nneurons: usize, p: f64) -> DropoutLayer {
        DropoutLayer {
            nneurons,
            p,
            probability_vector: Vector::new(nneurons),
            rng: rand::thread_rng(),
        }
    }

    pub(crate) fn initialize(&mut self, prev_layer: &Layer) {
        assert_eq!(self.nneurons, prev_layer.number_of_neurons());
    }

    pub fn number_of_neurons(&self) -> usize {
        self.nneurons
    }

    pub(crate) fn next_training_sample(&mut self) {
        let distribution = Uniform::new(0.0, 1.0);
        for idx in 0..self.probability_vector.dim() {
            let v = distribution.sample(&mut self.rng);
            self.probability_vector[idx] = 1.0 / v;
        }
    }

    pub(crate) fn feedforward(&self, prev_a: &Vector) -> Vector {
        let a = prev_a.hadamard(&self.probability_vector);
        a
    }

    pub fn backprop(&self, layer_index: usize, mb: &mut Minibatch) {
        assert!(layer_index > 0);
        let delta_next = &mb.error[layer_index + 1];
        let delta = delta_next.hadamard(&self.probability_vector);
        mb.error[layer_index] = delta;
    }

    pub(crate) fn print_summary(&self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {}

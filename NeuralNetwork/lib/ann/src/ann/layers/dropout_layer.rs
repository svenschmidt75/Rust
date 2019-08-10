use crate::ann::activation::Activation;
use crate::ann::layers::layer::Layer;
use crate::ann::minibatch::Minibatch;
use linear_algebra::matrix::Matrix2D;
use linear_algebra::vector::Vector;
use rand::distributions::{Bernoulli, Distribution};

pub struct DropoutLayer {
    p: f64,
    probability_vector: Vector,
}

impl DropoutLayer {
    pub(crate) fn initialize(&mut self, prev_layer: &Layer) {
        // generate vector with p and 0
        let distribution = Bernoulli::new(self.p);
        let mut rng = rand::thread_rng();

        let n_prev_a = prev_layer.NumberOfNeurons();
        let data = (0..n_prev_a)
            .into_iter()
            .map(|_| match distribution.sample(&mut rng) {
                // inverted dropout
                true => 1.0 / self.p,
                _ => 0.0,
            })
            .collect::<Vec<_>>();

        self.probability_vector = Vector::from(data);
    }

    pub fn NumberOfNeurons(&self) -> usize {
        self.probability_vector.dim()
    }

    pub(crate) fn feedforward(&self, prev_a: &Vector) -> Vector {
        let a = prev_a.hadamard(&self.probability_vector);
        a
    }

    pub fn backprop(&self, layer_index: usize, output_layer_index: usize, next_layer: &Layer, mb: &mut Minibatch) {
        assert!(layer_index > 0 && layer_index < output_layer_index);
        let delta_next = &mb.error[layer_index + 1];
        let delta = delta_next.hadamard(&self.probability_vector);
        mb.error[layer_index] = delta;
    }

    pub(crate) fn print_summary(&self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}

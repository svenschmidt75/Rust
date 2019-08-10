use crate::ann::activation::Activation;
use linear_algebra::vector::Vector;
use crate::ann::minibatch::Minibatch;
use crate::ann::layers::layer::Layer;

pub struct ActivationLayer {
    nneurons: usize,
    activation: Box<dyn Activation>,
}

impl ActivationLayer {
    pub fn new(nneurons: usize, activation: Box<dyn Activation>) -> ActivationLayer {
        ActivationLayer { nneurons, activation }
    }

    pub fn NumberOfNeurons(&self) -> usize {
        self.nneurons
    }

    pub(crate) fn feedforward(&self, z: &Vector) -> Vector {
        let a = self.activation.f(&z);
        a
    }

    pub(crate) fn print_summary(&self) {
        println!("summary");
    }

    pub fn backprop(&self, layer_index: usize, output_layer_index: usize, next_layer: &Layer, mb: &mut Minibatch) {
        assert!(layer_index > 0 && layer_index < output_layer_index);

        // SS: calculate da{l+1}/dz_{l}
        let delta_next = &mb.error[layer_index + 1];
        let z = &mb.input[layer_index];
        let sigma_prime = self.activation.df(z);
        let delta = delta_next.hadamard(&sigma_prime);
        mb.error[layer_index] = delta;
    }



}

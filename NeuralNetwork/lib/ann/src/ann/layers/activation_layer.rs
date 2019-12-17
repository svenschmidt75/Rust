use crate::ann::activation::Activation;
use crate::ann::minibatch::Minibatch;
use linear_algebra::vector::Vector;

pub struct ActivationLayer {
    nneurons: usize,
    activation: Box<dyn Activation>,
}

impl ActivationLayer {
    pub fn new(nneurons: usize, activation: Box<dyn Activation>) -> ActivationLayer {
        ActivationLayer { nneurons, activation }
    }

    pub fn number_of_neurons(&self) -> usize {
        self.nneurons
    }

    pub(crate) fn feedforward(&self, layer_index: usize, mbs: &mut [Minibatch]) {
        assert!(layer_index > 0);
        for mb in mbs {
            let input = &mb.output[layer_index - 1];
            let a = self.activation.f(input);
            mb.output[layer_index] = a;
        }
    }

    pub(crate) fn print_summary(&self) {
        println!("{:15} | {:>15} | {:15}", "activation", self.activation.id(), "");
    }

    pub fn backprop(&self, layer_index: usize, mbs: &mut [Minibatch]) {
        assert!(layer_index > 0);

        // SS: calculate da^{l+1}/dz^{l}
        for mb in mbs {
            let delta_next = &mb.error[layer_index + 1];
            let z = &mb.output[layer_index - 1];
            let sigma_prime = self.activation.df(z);
            let delta = delta_next.hadamard(&sigma_prime);
            mb.error[layer_index] = delta;
        }
    }
}

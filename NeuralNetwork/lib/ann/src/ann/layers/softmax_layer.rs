#![allow(non_snake_case)]

use linear_algebra::vector::Vector;

use crate::ann::layers::softmax::softmax;
use crate::ann::minibatch::Minibatch;

pub struct SoftMaxLayer {
    nneurons: usize,
}

impl SoftMaxLayer {
    pub fn new(nneurons: usize) -> SoftMaxLayer {
        SoftMaxLayer { nneurons }
    }

    pub fn number_of_neurons(&self) -> usize {
        self.nneurons
    }

    pub(crate) fn feedforward(&self, z: &Vector) -> Vector {
        let a = softmax(z);
        a
    }

    pub(crate) fn print_summary(&self) {
        println!("summary");
    }

    pub fn backprop(&self, layer_index: usize, mb: &mut Minibatch) {
        assert!(layer_index > 0);

        // SS: calculate da^{l+1}/dz^{l}

        // SS: dC/da^{l}
        let delta_next = &mb.error[layer_index + 1];
        let a = &mb.output[layer_index];
        let mut dC_dz = Vector::new(self.nneurons);
        for i in 0..self.nneurons {
            let mut result = 0.0;
            for k in 0..delta_next.dim() {
                let mut da_k_dz_i = -a[k] * a[i];
                if i == k {
                    da_k_dz_i += a[k];
                }
                result += da_k_dz_i * delta_next[k];
            }
            dC_dz[i] = result;
        }
        mb.error[layer_index] = dC_dz;
    }
}

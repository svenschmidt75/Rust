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

        // SS
        // 1. initialize matrix
        // 2. matrix multiplication

        // SS: write test for backprop!

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

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use linear_algebra::vector::Vector;

    use super::*;

    #[test]
    fn test_softmax_backprop1() {
        // Arrange
        let layer = SoftMaxLayer { nneurons: 2 };
        let mut mb = Minibatch::new(vec![2, 2, 2]);

        let z0 = 0.765;
        let z1 = 0.134;

        mb.output[0] = Vector::from(vec![z0, z1]);

        // calculate a from z
        mb.output[1] = layer.feedforward(&mb.output[0]);
        let a0 = mb.output[1][0];
        let a1 = mb.output[1][1];

        // C(a0, a1) = -3 * sin(a0) + 5 * cos(a1)

        // set dC/da
        let dCda0 = -3.0 * a0.cos();
        let dCda1 = -5.0 * a1.sin();
        mb.error[2] = Vector::from(vec![dCda0, dCda1]);

        // Act
        layer.backprop(1, &mut mb);

        // Assert
        // dCdz0 = dCda0 * da0dz0 + dCda1 * da1dz0
        let dCdz0 = dCda0 * a0 * (1.0 - a0) - dCda1 * a1 * a0;
        let dCdz1 = - dCda0 * a0 * a1 + dCda1 * a1 * (1.0 - a1);
        assert_approx_eq!(dCdz0, mb.error[1][0], 1E-12);
        assert_approx_eq!(dCdz1, mb.error[1][1], 1E-12);
    }
}

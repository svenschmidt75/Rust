#![allow(non_snake_case)]

use crate::ann::layers::softmax::softmax;
use crate::ann::minibatch::Minibatch;
use linear_algebra::matrix::Matrix2D;

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

    pub(crate) fn feedforward(&self, layer_index: usize, mbs: &mut [Minibatch]) {
        assert!(layer_index > 0);
        for mb in mbs {
            let input = &mb.output[layer_index - 1];
            let a = softmax(input);
            mb.output[layer_index] = a;
        }
    }

    pub(crate) fn print_summary(&self) {
        println!("{:15} | {:15} | {:15}", "softmax", "", "");
    }

    pub fn backprop(&self, layer_index: usize, mbs: &mut [Minibatch]) {
        assert!(layer_index > 0);

        // SS: calculate da^{l+1}/dz^{l}
        for mb in mbs {
            // SS: dC/da^{l}
            let delta_next = &mb.error[layer_index + 1];

            let a = &mb.output[layer_index];

            // SS: local_gradient = d_a/d_z
            let mut local_gradient = Matrix2D::new(self.nneurons, self.nneurons);
            for i in 0..self.nneurons {
                for k in 0..self.nneurons {
                    let mut da_k_dz_i = -a[k] * a[i];
                    if i == k {
                        da_k_dz_i += a[k];
                    }
                    local_gradient[(i, k)] = da_k_dz_i;
                }
            }

            // SS: multiply local gradient by incoming gradient
            let dC_dz = local_gradient.ax(delta_next);
            mb.error[layer_index] = dC_dz;
        }
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
        let layer = SoftMaxLayer { nneurons: 2 };
        let mut mb = Minibatch::new(vec![2, 2, 2]);
        let mut mbs = [mb];

        let z0 = 0.765;
        let z1 = 0.134;

        mbs[0].output[0] = Vector::from(vec![z0, z1]);

        // calculate a from z
        layer.feedforward(1, &mut mbs);
        let a0 = mbs[0].output[1][0];
        let a1 = mbs[0].output[1][1];

        // C(a0, a1) = -3 * sin(a0) + 5 * cos(a1)

        // set dC/da
        let dCda0 = -3.0 * a0.cos();
        let dCda1 = -5.0 * a1.sin();
        mbs[0].error[2] = Vector::from(vec![dCda0, dCda1]);

        // Act
        layer.backprop(1, &mut mbs);

        // Assert
        // dCdz0 = dCda0 * da0dz0 + dCda1 * da1dz0
        let dCdz0 = dCda0 * a0 * (1.0 - a0) - dCda1 * a1 * a0;
        let dCdz1 = -dCda0 * a0 * a1 + dCda1 * a1 * (1.0 - a1);
        assert_approx_eq!(dCdz0, mbs[0].error[1][0], 1E-12);
        assert_approx_eq!(dCdz1, mbs[0].error[1][1], 1E-12);
    }
}

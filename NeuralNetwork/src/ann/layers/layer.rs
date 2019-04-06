use crate::ann::activation::Activation;
use crate::la::matrix::Matrix2D;
use crate::la::ops;
use crate::la::vector::Vector;

pub trait Layer {
    //    fn initialize(); -- allocate memory for parameters
    //    fn on_start_new_epoch();
    fn feedforward(&self, a: &Vector) -> (Vector, Vector);

    fn nactivations(&self) -> usize;

    fn get_weights(&self) -> &Matrix2D;

    fn get_weights_mut(&mut self) -> &mut Matrix2D;

    fn get_biases_mut(&mut self) -> &mut Vector;

    fn get_activation(&self) -> &Activation;
}

pub struct FCLayer {
    weights: Matrix2D,
    biases: Vector,
    activation: Box<dyn Activation>,
}

impl FCLayer {
    pub fn new(weights: Matrix2D, biases: Vector, activation: Box<dyn Activation>) -> FCLayer {
        assert_eq!(weights.nrows(), biases.dim());
        FCLayer { weights, biases, activation }
    }

    fn get_weight(&self, i: usize, j: usize) -> f64 {
        // i: index of activation in layer l
        // j: index of activation in layer l-1
        self.weights.get(i, j)
    }
}

impl Layer for FCLayer {
    fn feedforward(&self, input: &Vector) -> (Vector, Vector) {
        // SS: number of activations in this layer: self.weights.nrows()
        let output = ops::ax(&self.weights, input);

        // SS: alternatively, add another column to weights with the biases.
        // Add another row with all 0s, except for the bias column where we put 1.
        let z = &output + &self.biases;
        let a = self.activation.f(&z);
        (a, z)
    }

    fn nactivations(&self) -> usize {
        self.biases.dim()
    }

    fn get_weights(&self) -> &Matrix2D {
        &self.weights
    }

    fn get_weights_mut(&mut self) -> &mut Matrix2D {
        &mut self.weights
    }

    fn get_biases_mut(&mut self) -> &mut Vector {
        &mut self.biases
    }

    fn get_activation(&self) -> &Activation {
        &*self.activation
    }
}

pub struct InputLayer {
    nactivations: usize,
}

impl InputLayer {
    pub fn new(nactivations: usize) -> InputLayer {
        InputLayer { nactivations }
    }
}

impl Layer for InputLayer {
    fn feedforward(&self, _a: &Vector) -> (Vector, Vector) {
        unreachable!()
    }

    fn nactivations(&self) -> usize {
        self.nactivations
    }

    fn get_weights(&self) -> &Matrix2D {
        unreachable!()
    }

    fn get_weights_mut(&mut self) -> &mut Matrix2D {
        unreachable!()
    }

    fn get_biases_mut(&mut self) -> &mut Vector {
        unreachable!()
    }

    fn get_activation(&self) -> &Activation {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    //    use super::*;

    #[test]
    fn test_index() {}
}

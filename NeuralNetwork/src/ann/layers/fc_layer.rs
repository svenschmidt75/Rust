use crate::ann::layers::layer::Layer;
use crate::la::matrix::Matrix;
use crate::la::vector::Vector;
use crate::la::ops;
use crate::ann::activation::Activation;

pub struct FCLayer {
    weights: Matrix,
    biases: Vector,
    activation: Box<dyn Activation>
}

impl FCLayer {

    pub fn new(weights: Matrix, biases: Vector, activation: Box<dyn Activation>) -> FCLayer {
        FCLayer { weights, biases, activation }
    }

    fn get_weight(&self, i: usize, j: usize) -> f64 {
        // i: index of activation in layer l
        // j: index of activation in layer l-1
        self.weights.get(i, j)
    }

}

impl Layer for FCLayer {

    fn feedforward(&self, input: &Vector) -> Vector {
        // SS: number of activations in this layer: self.weights.nrows()
        let mut output = ops::ax(&self.weights, input);

        // SS: alternatively, add another column to weights with the biases.
        // Add another row with all 0s, except for the bias column where we put 1.
        output = &output + &self.biases;
        output = self.activation.f(output);
        output
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {}
}

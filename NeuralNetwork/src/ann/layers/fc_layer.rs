use crate::ann::layers::layer::Layer;
use crate::la::matrix::Matrix;
use crate::la::vector::Vector;
use crate::la::ops;

pub struct FCLayer {
    weights: Matrix,
    biases: Vector
}

impl FCLayer {

    pub fn new(weights: Matrix, biases: Vector) -> FCLayer {
        FCLayer { weights, biases }
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
        output = &output + &self.biases;
        output
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {}
}

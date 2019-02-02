use crate::ann::layers::layer::Layer;
use crate::la::matrix::Matrix;
use crate::la::vector::Vector;

pub struct FCLayer {
    weights: Matrix
}

impl FCLayer {

    pub fn new(weights: Matrix) -> FCLayer {
        FCLayer { weights }
    }

    fn get_weight(&self, i: usize, j: usize) -> f64 {
        // i: index of activation in layer l
        // j: index of activation in layer l-1
        self.weights.get(i, j)
    }

}

impl Layer for FCLayer {

    fn feedforward(&self, input: &Vector) -> Vector {
        unimplemented!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {}
}

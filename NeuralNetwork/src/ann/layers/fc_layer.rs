use crate::ann::layers::layer::Layer;
use crate::la::matrix::Matrix;

struct FCLayer {
    weights: Matrix
}

impl FCLayer {

    fn get_weight(&self, i: usize, j: usize) -> f64 {
        // i: index of activation in layer l
        // j: index of activation in layer l-1
        self.weights.get(i, j)
    }

}

impl Layer for FCLayer {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {}
}

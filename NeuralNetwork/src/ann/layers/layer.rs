use crate::la::vector::Vector;
use crate::la::matrix::Matrix;
use crate::ann::activation::Activation;

pub trait Layer {
    //    fn initialize(); -- allocate memory for parameters
    //    fn on_start_new_epoch();
    fn feedforward(&self, a: &Vector) -> (Vector, Vector);

    fn backpropagate(&self, error: &Vector) -> Vector;

    fn nactivations(&self) -> usize;

    fn get_weights(&self) -> &Matrix;

    fn get_activation(&self) -> &Activation;
}

use crate::ann::activation::Activation;
use crate::la::matrix::Matrix;
use crate::la::vector::Vector;

pub trait Layer {
    //    fn initialize(); -- allocate memory for parameters
    //    fn on_start_new_epoch();
    fn feedforward(&self, a: &Vector) -> (Vector, Vector);

    fn nactivations(&self) -> usize;

    fn get_weights(&self) -> &Matrix;

    fn get_activation(&self) -> &Activation;
}

use crate::ann::activation::Activation;
use crate::ann::layers::layer::Layer;
use linear_algebra::matrix::Matrix2D;
use linear_algebra::vector::Vector;

pub struct InputLayer {
    pub nactivations: usize,
}

impl InputLayer {
    pub fn new(nactivations: usize) -> InputLayer {
        InputLayer { nactivations }
    }
}

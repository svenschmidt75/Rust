use crate::ann::layers::layer::Layer;
use crate::la::vector::Vector;

struct InputLayer {
    size: usize
}

impl Layer for InputLayer {
    fn set_activations(&self, a: &Vector) {
        unimplemented!()
    }
}

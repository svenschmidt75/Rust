use crate::la::vector::Vector;

pub trait Layer {
    fn set_activations(&self, a: &Vector);
}

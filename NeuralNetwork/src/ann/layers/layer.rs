use crate::la::vector::Vector;

pub trait Layer {
//    fn initialize(); -- allocate memory for parameters
//    fn on_start_new_epoch();
    fn set_activations(&mut self, a: &Vector);
}

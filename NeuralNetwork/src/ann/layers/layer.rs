use crate::la::vector::Vector;

pub trait Layer {
    //    fn initialize(); -- allocate memory for parameters
    //    fn on_start_new_epoch();
    fn feedforward(&self, a: &Vector) -> (Vector, Vector);

    fn nactivations(&self) -> usize;
}

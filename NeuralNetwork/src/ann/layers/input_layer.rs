use crate::ann::layers::layer::Layer;
use crate::la::vector::Vector;

struct InputLayer {
    a: Vector,
    size: usize
}

impl Layer for InputLayer {
    fn set_activations(&mut self, a: &Vector) {
        assert_eq!(self.a.dim(), a.dim());
        self.a = (*a).clone();
    }

}

use crate::la::vector::Vector;

pub struct Minibatch {
     a: Vec<Vector>,
     error: Vec<Vector>,
}

impl Minibatch {

     pub fn new() -> Minibatch {
          Minibatch { a: vec![], error: vec![] }
     }

     pub fn activation(&self, layer_index: usize) -> &Vector {
          &self.a[layer_index]
     }

     pub fn add_activation(&mut self, a: Vector) {
          self.a.push(a);
     }

}
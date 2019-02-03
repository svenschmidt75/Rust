use crate::la::vector::Vector;

pub struct Minibatch {
    a: Vec<Vector>,
    error: Vec<Vector>,
}

impl Minibatch {
    pub fn new() -> Minibatch {
        Minibatch {
            a: vec![],
            error: vec![],
        }
    }

    pub fn activation(&self, layer_index: usize) -> &Vector {
        &self.a[layer_index]
    }

    pub fn output_activation(&self) -> &Vector {
        self.activation(self.a.len() - 1)
    }

    pub fn set_input_activation(&mut self, a: Vector) {
        if self.a.len() < 1 {
            self.a.push(a);
        } else {
            self.a[0] = a;
        }
    }

    pub fn add_activation(&mut self, a: Vector) {
        self.a.push(a);
    }
}

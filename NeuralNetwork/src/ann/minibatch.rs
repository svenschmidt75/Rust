use crate::la::vector::Vector;

pub struct Minibatch {
    a: Vec<Vector>,
    z: Vec<Vector>,
    error: Vec<Vector>,
}

impl Minibatch {
    pub fn new(nas: Vec<usize>) -> Minibatch {
        let acts: Vec<Vector> = nas.iter().map(|&na| Vector::new(na)).collect();
        let zs: Vec<Vector> = nas.iter().map(|&na| Vector::new(na)).collect();
        let errors: Vec<Vector> = nas.iter().map(|&na| Vector::new(na)).collect();
        Minibatch {
            a: acts,
            z: zs,
            error: errors,
        }
    }

    pub fn a(&self, layer_index: usize) -> &Vector {
        &self.a[layer_index]
    }

    pub fn z(&self, layer_index: usize) -> &Vector {
        &self.z[layer_index]
    }

    pub fn set_input_a(&mut self, a: Vector) {
        self.a[0] = a;
    }

    pub fn output_activations(&self) -> &Vector {
        &self.a[self.a.len() - 1]
    }

    pub fn store(&mut self, layer_index: usize, a: Vector, z: Vector) {
        self.a[layer_index] = a;
        self.z[layer_index] = z;
    }

}

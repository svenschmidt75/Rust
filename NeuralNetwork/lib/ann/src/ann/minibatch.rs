use linear_algebra::vector::Vector;

pub struct Minibatch {
    pub a: Vec<Vector>,
    pub z: Vec<Vector>,
    pub error: Vec<Vector>,
}

impl Minibatch {
    pub fn new(nas: Vec<usize>) -> Minibatch {
        let acts: Vec<Vector> = nas.iter().map(|&na| Vector::new(na)).collect();
        let zs: Vec<Vector> = nas.iter().map(|&na| Vector::new(na)).collect();
        let errors: Vec<Vector> = nas.iter().map(|&na| Vector::new(na)).collect();
        Minibatch { a: acts, z: zs, error: errors }
    }

    pub fn output_activations(&self) -> &Vector {
        &self.a[self.a.len() - 1]
    }
}

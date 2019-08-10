use linear_algebra::vector::Vector;

pub struct Minibatch {
    pub input: Vec<Vector>,
    pub error: Vec<Vector>,
}

impl Minibatch {
    pub fn new(nas: Vec<usize>) -> Minibatch {
        let zs: Vec<Vector> = nas.iter().map(|&na| Vector::new(na)).collect();
        let errors: Vec<Vector> = nas.iter().map(|&na| Vector::new(na)).collect();
        Minibatch { input: zs, error: errors }
    }

    pub fn output_activations(&self) -> &Vector {
        &self.input[self.input.len() - 1]
    }
}

use linear_algebra::vector::Vector;

pub struct Minibatch {
    pub output: Vec<Vector>,
    pub error: Vec<Vector>,
}

impl Minibatch {
    pub fn new(nas: Vec<usize>) -> Minibatch {
        let zs: Vec<Vector> = nas.iter().map(|&na| Vector::new(na)).collect();
        let errors: Vec<Vector> = nas.iter().map(|&na| Vector::new(na)).collect();
        Minibatch { output: zs, error: errors }
    }

    pub fn output_activations(&self) -> &Vector {
        // SS: -2 because -1 is the error dCda
        &self.output[self.output.len() - 2]
    }

}

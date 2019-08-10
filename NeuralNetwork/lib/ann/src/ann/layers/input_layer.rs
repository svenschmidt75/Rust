pub struct InputLayer {
    nneurons: usize,
}

impl InputLayer {
    pub fn new(nneurons: usize) -> InputLayer {
        InputLayer { nneurons }
    }

    pub fn number_of_neurons(&self) -> usize {
        self.nneurons
    }
}

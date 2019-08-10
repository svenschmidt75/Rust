pub struct InputLayer {
    nneurons: usize,
}

impl InputLayer {
    pub fn new(nneurons: usize) -> InputLayer {
        InputLayer { nneurons }
    }

    pub fn NumberOfNeurons(&self) -> usize {
        self.nneurons
    }
}

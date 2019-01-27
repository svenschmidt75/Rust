use crate::ann::layers::layer;

struct Model {
    layers: Box<dyn layer::Layer>
}

impl Model {

    pub fn train(&self) {

    }

    fn feedforward(&self) {

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {}
}

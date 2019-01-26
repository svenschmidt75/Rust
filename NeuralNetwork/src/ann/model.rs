use crate::ann::layers::layer;

struct Model {
    layers: Box<dyn layer::Layer>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {}
}

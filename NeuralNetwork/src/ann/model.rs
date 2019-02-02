use crate::ann::layers::layer::Layer;
use crate::ann::layers::training_data::TrainingData;
use crate::ann::minibatch::Minibatch;

struct Model {
    layers: Vec<Box<dyn Layer>>
}

impl Model {
    fn new() -> Model {
        Model { layers: vec![] }
    }

    pub fn add(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer)
    }

    // pass in training and validation data
    // number of epochs
    // size of minibatch
    // no regularization for now
    pub fn train(&self) {
        // call initialize on each layer

        // for each epoch
        // shuffle training data indices
        // for each minibatch
        //   call on_new_epoch on all layers
        //   feed forward
        //   calculate error in output layer
        //   backprop
        //   update parameters
        // print statistics
    }

    pub fn feedforward(&mut self, mb: &mut Minibatch) {
        // SS: feed forward one instance of a training data sample
        // and record all calculated activations for all layers
        // for backprop.
        for i in 0..(self.layers.len()) {
            let layer = &self.layers[i];
            let output = layer.feedforward(mb.activation(i));
            mb.add_activation(output);
        }
    }

    pub fn summary(&self) {
        // print out number of layers, number of parameters, etc.
    }
}

#[cfg(test)]
mod tests {
    use crate::ann::layers::fc_layer::FCLayer;
    use crate::la::matrix::Matrix;

    use super::*;
    use crate::la::vector::Vector;

    #[test]
    fn test_index() {
        // Arrange
        let mut model = Model::new();
        let weights = Matrix::new_from_data(3, 2, vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6]);
        let hidden_layer = FCLayer::new(weights);
        model.add(Box::new(hidden_layer));

        // todo SS: add activation function

        let weights = Matrix::new_from_data(1, 3, vec![0.1, 0.2, 0.3]);
        let output_layer = FCLayer::new(weights);
        model.add(Box::new(output_layer));

        let mut mb = Minibatch::new();
        mb.add_activation(Vector::from(vec![0.0, 1.0]));

        // Act
        model.feedforward(&mut mb);

    }
}

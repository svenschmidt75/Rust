use crate::ann::cost_function::CostFunction;
use crate::ann::layers::layer::Layer;
use crate::ann::layers::training_data::TrainingData;
use crate::ann::minibatch::Minibatch;

pub struct Model {
    layers: Vec<Box<dyn Layer>>,
}

impl Model {
    pub fn new() -> Model {
        Model { layers: vec![] }
    }

    pub fn add(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer)
    }

    pub fn nlayers(&self) -> usize {
        self.layers.len()
    }

    // pass in training and validation data
    // number of epochs
    // size of minibatch
    // no regularization for now
    pub fn train(
        &mut self,
        training_data: &Vec<TrainingData>,
        validation_data: &Vec<TrainingData>,
        epochs: usize,
        minibatch_size: usize,
        cost_function: Box<dyn CostFunction>,
    ) {
        // call initialize on each layer

        // for each epoch
        //   shuffle training data indices
        //   for each minibatch
        //     call on_new_epoch on all layers
        //     feed forward
        //     calculate error in output layer
        //   backprop
        //   update parameters
        // print statistics
    }

    pub fn create_minibatch(&self) -> Minibatch {
        let mut nas: Vec<_> = self.layers.iter().map(|layer| layer.na()).collect();
        nas.insert(0, self.layers[0].na());
        Minibatch::new(nas)
    }

    pub fn feedforward(&mut self, mb: &mut Minibatch) {
        // SS: feed forward one instance of a training data sample
        // and record all calculated activations for all layers
        // for backprop.
        for layer_index in 0..(self.layers.len()) {
            let layer = &self.layers[layer_index];
            let (a, z) = layer.feedforward(mb.a(layer_index));
            mb.store(layer_index + 1, a, z);
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
    use crate::ann::activation;
    use crate::ann::activation::ReLU;
    use crate::ann::activation::Sigmoid;
    use crate::la::vector::Vector;

    #[test]
    fn test_feedforward() {
        // Arrange
        let mut model = Model::new();
        let weights1 = Matrix::new_from_data(3, 2, vec![0.0, 0.01, 0.02, 0.10, 0.11, 0.12]);
        let biases1: Vector = vec![0.1, 0.2, 0.3].into();
        let hidden_layer = FCLayer::new(weights1.clone(), biases1.clone(), Box::new(Sigmoid {}));
        model.add(Box::new(hidden_layer));

        let weights2 = Matrix::new_from_data(1, 3, vec![0.1, 0.2, 0.3]);
        let biases2: Vector = vec![0.1].into();
        let output_layer = FCLayer::new(weights2.clone(), biases2.clone(), Box::new(ReLU {}));
        model.add(Box::new(output_layer));

        let mut mb = model.create_minibatch();
        mb.store(0, Vector::from(vec![0.0, 1.0]), Vector::from(vec![0.0, 1.0]));

        // Act
        model.feedforward(&mut mb);

        // Assert

        // a^{1}_{0}
        let a10 = activation::sigmoid(
            weights1.get(0, 0) * mb.a(0)[0]
                + weights1.get(0, 1) * mb.a(0)[1]
                + biases1[0],
        );
        assert_eq!(a10, mb.a(1)[0]);

        // a^{1}_{1}
        let a11 = activation::sigmoid(
            weights1.get(1, 0) * mb.a(0)[0]
                + weights1.get(1, 1) * mb.a(0)[1]
                + biases1[1],
        );
        assert_eq!(a11, mb.a(1)[1]);

        // a^{1}_{2}
        let a12 = activation::sigmoid(
            weights1.get(2, 0) * mb.a(0)[0]
                + weights1.get(2, 1) * mb.a(0)[1]
                + biases1[2],
        );
        assert_eq!(a12, mb.a(1)[2]);

        // a^{2}_{0}
        let a20 = activation::relu(
            weights2.get(0, 0) * mb.a(1)[0]
                + weights2.get(0, 1) * mb.a(1)[1]
                + weights2.get(0, 2) * mb.a(1)[2]
                + biases2[0],
        );
        assert_eq!(a20, mb.a(2)[0]);
    }
}

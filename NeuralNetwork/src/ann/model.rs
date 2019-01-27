use crate::ann::layers::layer::Layer;
use crate::ann::layers::training_data::TrainingData;
use crate::ann::minibatch::Minibatch;

struct Model {
    layers: Vec<Box<dyn Layer>>
}

impl Model {

    // pass in training and validation data
    // number of epochs
    // size of minibatch
    // no regularization for now
    pub fn train(&self) {
        // for each epoch
        // shuffle training data indices
        // for each minibatch
        //   feed forward
        //   calculate error in output layer
        //   backprop
        //   update parameters
        // print statistics
    }

    fn feedforward(&mut self, x: &TrainingData, mb: &mut Minibatch) {
        // initialize input layer with training data
        // for all layers
        //   feed forward to next layer
        self.layers[0].set_activations(&x.input_activations);



    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {}
}

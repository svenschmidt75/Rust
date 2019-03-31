use crate::ann::activation::{Activation, Id};
use crate::ann::cost_function::CostFunction;
use crate::ann::layers::fc_layer::FCLayer;
use crate::ann::layers::layer::Layer;
use crate::ann::layers::training_data::TrainingData;
use crate::ann::minibatch::Minibatch;
use crate::la::matrix::Matrix2D;
use crate::la::ops;
use crate::la::vector::Vector;

pub struct Model {
    layers: Vec<Box<dyn Layer>>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            layers: vec![Box::new(FCLayer::new(
                Matrix2D::new(0, 0),
                Vector::new(0),
                Box::new(Id {}),
            ))],
        }
    }

    pub fn add(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer)
    }

    pub fn nlayers(&self) -> usize {
        self.layers.len()
    }

    pub fn output_layer_index(&self) -> usize {
        self.layers.len() - 1
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
        cost_function: &CostFunction,
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
        let mut nas: Vec<_> = self
            .layers
            .iter()
            .map(|layer| layer.nactivations())
            .collect();
        Minibatch::new(nas)
    }

    pub fn feedforward(&mut self, mb: &mut Minibatch) {
        // SS: feed forward one instance of a training data sample
        // and record all calculated activations for all layers
        // for backprop.
        let output_layer_index = self.output_layer_index();
        for layer_index in 0..output_layer_index {
            let layer = &self.layers[layer_index + 1];
            let (a, z) = layer.feedforward(&mb.a[layer_index]);
            mb.a[layer_index + 1] = a;
            mb.z[layer_index + 1] = z;
        }
    }

    pub fn calculate_outputlayer_error(
        &self,
        mb: &mut Minibatch,
        cost_function: &CostFunction,
        y: &Vector,
    ) {
        // SS: calculate delta_{L}, the error in the output layer
        let output_layer_index = self.output_layer_index();
        let layer = &self.layers[output_layer_index];
        let sigma = layer.get_activation();
        let output_error = cost_function.output_error(output_layer_index, mb, y, sigma);
        mb.error[output_layer_index] = output_error;
    }

    pub fn backprop(&mut self, mb: &mut Minibatch, cost_function: &CostFunction, y: &Vector) {
        // SS: backprop delta_{L}
        // start at index l=L-1
        self.calculate_outputlayer_error(mb, cost_function, &y);
        let output_layer_index = self.output_layer_index();
        for i in 0..output_layer_index - 1 {
            let layer_index = output_layer_index - i - 1;
            let layer = &self.layers[layer_index];
            let layer_next = &self.layers[layer_index + 1];

            let delta_next = &mb.error[layer_index + 1];
            let w_next = layer_next.get_weights().transpose();
            let z = &mb.z[layer_index];
            let sigma_prime = layer.get_activation().df(z);
            let delta_l = w_next.ax(delta_next).hadamard(&sigma_prime);
            mb.error[layer_index] = delta_l;
        }
    }

    pub fn calculate_derivatives(&self, mbs: &[&Minibatch]) -> (Vec<Matrix2D>, Vec<Vector>) {
        // TODO SS: Can the calculation of the derivatives be done in parallel,
        // followed by a reduction step to sum them up?

        let mut dws = Vec::<Matrix2D>::with_capacity(mbs.len());
        let mut dbs = Vec::<Vector>::with_capacity(mbs.len());

        let output_layer_index = self.output_layer_index();
        for i in 0..output_layer_index - 1 {
            let layer_index = output_layer_index - i - 1;

            let nactivations = self.layers[layer_index].nactivations();
            let nactivations_prev = self.layers[layer_index - 1].nactivations();

            let mut dCdw = Matrix2D::new(nactivations, nactivations_prev);
            let mut dCdb = Vector::new(nactivations);

            for mb_index in 0..mbs.len() {
                let mb = mbs[mb_index];
                let delta_i = &mb.error[layer_index];
                let a_j = &mb.a[layer_index - 1];

                let dw_ij = ops::outer_product(delta_i, a_j);
                dCdw += &dw_ij;

                let db_i = delta_i;
                dCdb += &db_i;
            }
            dCdw /= mbs.len();
            dws.push(dCdw);
            dCdb /= mbs.len();
            dbs.push(dCdb);
        }
        (dws, dbs)
    }

    pub fn summary(&self) {
        // print out number of layers, number of parameters, etc.
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::ann::activation;
    use crate::ann::activation::ReLU;
    use crate::ann::activation::Sigmoid;
    use crate::ann::cost_function::QuadraticCost;
    use crate::ann::layers::fc_layer::FCLayer;
    use crate::la::matrix::Matrix;
    use crate::la::matrix::Matrix2D;
    use crate::la::vector::Vector;

    use super::*;

    #[test]
    fn test_feedforward() {
        // Arrange
        let mut model = Model::new();
        let weights1 = Matrix2D::new_from_data(3, 2, vec![0.0, 0.01, 0.02, 0.10, 0.11, 0.12]);
        let biases1: Vector = vec![0.1, 0.2, 0.3].into();
        let hidden_layer = FCLayer::new(weights1.clone(), biases1.clone(), Box::new(Sigmoid {}));
        model.add(Box::new(hidden_layer));

        let weights2 = Matrix2D::new_from_data(1, 3, vec![0.1, 0.2, 0.3]);
        let biases2: Vector = vec![0.1].into();
        let output_layer = FCLayer::new(weights2.clone(), biases2.clone(), Box::new(ReLU {}));
        model.add(Box::new(output_layer));

        let mut mb = model.create_minibatch();
        mb.a[0] = Vector::from(vec![0.0, 1.0]);
        mb.z[0] = Vector::from(vec![0.0, 1.0]);

        // Act
        model.feedforward(&mut mb);

        // Assert

        // a^{1}_{0}
        let a10 = activation::sigmoid(
            weights1.get(0, 0) * mb.a[0][0] + weights1.get(0, 1) * mb.a[0][1] + biases1[0],
        );
        assert_eq!(a10, mb.a[1][0]);

        // a^{1}_{1}
        let a11 = activation::sigmoid(
            weights1.get(1, 0) * mb.a[0][0] + weights1.get(1, 1) * mb.a[0][1] + biases1[1],
        );
        assert_eq!(a11, mb.a[1][1]);

        // a^{1}_{2}
        let a12 = activation::sigmoid(
            weights1.get(2, 0) * mb.a[0][0] + weights1.get(2, 1) * mb.a[0][1] + biases1[2],
        );
        assert_eq!(a12, mb.a[1][2]);

        // a^{2}_{0}
        let a20 = activation::relu(
            weights2.get(0, 0) * mb.a[1][0]
                + weights2.get(0, 1) * mb.a[1][1]
                + weights2.get(0, 2) * mb.a[1][2]
                + biases2[0],
        );
        assert_eq!(a20, mb.a[2][0]);
    }

    #[test]
    fn test_backprop_errors() {
        // Arrange
        let mut model = Model::new();
        let weights1 = Matrix2D::new_from_data(2, 2, vec![0.1, 0.1, 0.1, 0.1]);
        let biases1: Vector = vec![0.2, 0.2].into();
        let hidden_layer = FCLayer::new(weights1.clone(), biases1.clone(), Box::new(Sigmoid {}));
        model.add(Box::new(hidden_layer));

        let weights2 = Matrix2D::new_from_data(1, 2, vec![0.3, 0.3]);
        let biases2: Vector = vec![0.4].into();
        let output_layer = FCLayer::new(weights2.clone(), biases2.clone(), Box::new(Sigmoid {}));
        model.add(Box::new(output_layer));

        let mut mb = model.create_minibatch();
        mb.z[0] = Vector::from(vec![0.0, 1.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);

        // expected output
        let y = Vector::from(vec![0.0]);

        model.feedforward(&mut mb);

        // Act
        model.backprop(&mut mb, &QuadraticCost {}, &y);

        // Assert
        assert_approx_eq!(0.7480485918792308, mb.z[2][0], 1e-5f64);
        assert_approx_eq!(0.3231058578630005, mb.z[1][0], 1e-5f64);
        assert_approx_eq!(0.3231058578630005, mb.z[1][1], 1e-5f64);
    }

    #[test]
    fn test_calculate_derivatives() {
        // Arrange

        // Act

        // Assert
    }

}

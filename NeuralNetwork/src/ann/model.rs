use std::cmp;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::ann::activation::Activation;
use crate::ann::cost_function::CostFunction;
use crate::ann::cost_function::QuadraticCost;
use crate::ann::layers::layer::Layer;
use crate::ann::layers::training_data::TrainingData;
use crate::ann::minibatch::Minibatch;
use crate::la::matrix::Matrix2D;
use crate::la::ops;
use crate::la::vector::Vector;

pub struct Model {
    layers: Vec<Box<dyn Layer>>,
}

fn get_neighbors2<U, T>(v: &mut Vec<T>, idx: usize) -> (&U, &mut U)
where
    T: Sized + AsRef<U> + AsMut<U>,
    U: ?Sized,
{
    let (l, r) = v.split_at_mut(idx);
    let item1 = l.last().unwrap().as_ref();
    let item2 = r[0].as_mut();
    (item1, item2)
}

impl Model {
    pub fn new() -> Model {
        Model { layers: vec![] }
    }

    pub fn add(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer)
    }

    pub fn get_layer(&self, idx: usize) -> &Layer {
        self.layers[idx].as_ref()
    }

    pub fn nlayers(&self) -> usize {
        self.layers.len()
    }

    pub fn output_layer_index(&self) -> usize {
        self.layers.len() - 1
    }

    pub fn get_weights(&self, layer_index: usize) -> &Matrix2D {
        &self.layers[layer_index].get_weights()
    }

    pub fn get_biases(&self, layer_index: usize) -> &Vector {
        &self.layers[layer_index].get_biases()
    }

    fn number_of_minibatches(training_data_size: usize, minibatch_size: usize) -> (usize, usize) {
        let n_minibatches = training_data_size / minibatch_size;
        (n_minibatches, training_data_size % minibatch_size)
    }

    // pass in training and validation data
    // number of epochs
    // size of minibatch
    // no regularization for now
    pub fn train(&mut self, data: &(&Vec<TrainingData>, &Vec<TrainingData>, &Vec<TrainingData>), epochs: usize, eta: f64, _lambda: f64, minibatch_size: usize, cost_function: &CostFunction) {
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

        // print update step after each epoch

        self.initialize_layers();

        let training_data = data.0;
        let mut trainingdata_indices: Vec<_> = (0..training_data.len()).collect();
        let mut rng = thread_rng();

        // SS: training samples per minibatch
        let mb_size = cmp::min(training_data.len(), minibatch_size);
        let mut mbs = (0..mb_size).map(|_| self.create_minibatch()).collect::<Vec<_>>();

        let (n_minibatches, remainder) = Model::number_of_minibatches(trainingdata_indices.len(), mb_size);

        println!("{:15} | {:15} | {:15}", "layer type", "shape", "param #");
        println!("{:->15} | {:->15} | {:->15}", "-", "-", "-");
        for layer in self.layers.iter().skip(1) {
            layer.print_summary();
        }
        println!("{:->15} | {:->15} | {:->15}", "-", "-", "-");
        println!();

        println!("Number of training samples: {}", data.0.len());
        println!("Number of validation samples: {}", data.1.len());
        println!("Number of test samples: {}", data.2.len());
        println!();

        println!("Minibatch size: {}", mb_size);
        println!("Number of minibatches: {}", n_minibatches);
        println!();

        println!("Learning rate: {}", eta);
        println!("Number of epochs: {}", epochs);
        println!("L2 regularization: {}", _lambda);
        println!();

        for _epoch in 0..epochs {
            // random shuffle on training data
            trainingdata_indices.shuffle(&mut rng);

            // SS: Split the training data into chunks, each the size of a minibatch.
            // #chunks = mb_size, plus remainder
            let chunks = trainingdata_indices.chunks(mb_size);

            for chunk in chunks {
                //                println!("{:?}", chunk);
                for idx in 0..chunk.len() {
                    let mb = &mut mbs[idx];
                    let training_sample_idx = chunk[idx];
                    let training_sample = &training_data[training_sample_idx];
                    let known_classification = &training_sample.output_activations;
                    mb.a[0] = training_sample.input_activations.clone();
                    self.feedforward(mb);
                    self.backprop(mb, cost_function, known_classification);
                }
                let (dws, dbs) = self.calculate_derivatives(&mbs[..]);
                self.update_network(eta, dws, dbs);
            }

            // SS: epoch completed, print statistics
        }
    }

    fn initialize_layers(&mut self) {
        for idx in 1..self.layers.len() {
            let (prev, current) = get_neighbors2(&mut self.layers, idx);
            current.initialize(prev);
        }
    }

    pub fn create_minibatch(&self) -> Minibatch {
        let nas: Vec<_> = self.layers.iter().map(|layer| layer.nactivations()).collect();
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

    pub fn calculate_outputlayer_error(&self, mb: &mut Minibatch, cost_function: &CostFunction, y: &Vector) {
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

    pub fn calculate_derivatives(&self, mbs: &[Minibatch]) -> (Vec<Matrix2D>, Vec<Vector>) {
        // TODO SS: Can the calculation of the derivatives be done in parallel,
        // followed by a reduction step to sum them up?

        let mut dws = Vec::<Matrix2D>::with_capacity(mbs.len());
        let mut dbs = Vec::<Vector>::with_capacity(mbs.len());

        let output_layer_index = self.output_layer_index();
        for layer_index in 1..output_layer_index + 1 {
            let nactivations = self.layers[layer_index].nactivations();
            let nactivations_prev = self.layers[layer_index - 1].nactivations();

            let mut dCdw = Matrix2D::new(nactivations, nactivations_prev);
            let mut dCdb = Vector::new(nactivations);

            for mb_index in 0..mbs.len() {
                let mb = &mbs[mb_index];
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

    fn update_network(&mut self, eta: f64, dws: Vec<Matrix2D>, dbs: Vec<Vector>) {
        let output_layer_index = self.output_layer_index();
        assert_eq!(output_layer_index, dws.len());
        assert_eq!(output_layer_index, dbs.len());

        // SS: move this into layer class?
        // Reason 1: get_weights() does not need to be mutable
        // Reason 2: for CNN, the weights matrix is sparse

        // SS: dws and dbs contain the layer 1 deltas at index 0!
        for layer_index in 1..output_layer_index + 1 {
            {
                let weights = self.layers[layer_index].get_weights_mut();
                let dw = &dws[layer_index - 1];
                *weights -= &(eta * dw);
            }
            {
                let biases = self.layers[layer_index].get_biases_mut();
                let db = &dbs[layer_index - 1];
                *biases -= &(eta * db);
            }
        }
    }

    pub fn summary(&self) {}

    fn calculate_delta(model: &Model, layer_index: usize, mb: &Minibatch, x: &TrainingData) -> Vector {
        // SS: same as backprop, but here we are using recursion
        let output_layer_index = model.output_layer_index();
        if layer_index == output_layer_index {
            let layer = model.get_layer(layer_index);
            let sigma_prime = layer.get_activation().df(&mb.z[layer_index]);
            return (&mb.a[layer_index] - &x.output_activations).hadamard(&sigma_prime);
        }
        let delta_next = Model::calculate_delta(&model, layer_index + 1, &mb, &x);
        let w_tr = model.get_weights(layer_index).transpose();
        let layer = model.get_layer(layer_index);
        let sigma_prime = layer.get_activation().df(&mb.z[layer_index]);
        w_tr.ax(&delta_next).hadamard(&sigma_prime)
    }

    fn grad_bias(model: &mut Model, layer_index: usize, xs: &[TrainingData]) -> Vector {
        // SS: same as calculate_derivatives, but here we are using recursion
        assert!(layer_index > 0);
        let layer = model.get_layer(layer_index);
        let mut db = Vector::new(layer.nactivations());
        let mut mb = model.create_minibatch();

        for training_sample in xs {
            let known_classification = &training_sample.output_activations;
            mb.a[0] = training_sample.input_activations.clone();
            model.feedforward(&mut mb);
            let delta = Model::calculate_delta(&model, layer_index, &mb, &training_sample);
            db += &delta;
        }
        db /= xs.len();
        db
    }

    fn grad_weight(model: &mut Model, layer_index: usize, xs: &[TrainingData]) -> Matrix2D {
        // SS: same as calculate_derivatives, but here we are using recursion
        assert!(layer_index > 0);
        let prev_layer = model.get_layer(layer_index - 1);
        let layer = model.get_layer(layer_index);
        let mut dw = Matrix2D::new(layer.nactivations(), prev_layer.nactivations());
        let mut mb = model.create_minibatch();

        for training_sample in xs {
            let known_classification = &training_sample.output_activations;
            mb.a[0] = training_sample.input_activations.clone();
            model.feedforward(&mut mb);
            let delta = Model::calculate_delta(&model, layer_index, &mb, &training_sample);
            let tmp = ops::outer_product(&delta, &mb.a[layer_index - 1]);
            dw += &tmp;
        }
        dw /= xs.len();
        dw
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::ann::activation;
    use crate::ann::activation::ReLU;
    use crate::ann::activation::Sigmoid;
    use crate::ann::cost_function::QuadraticCost;
    use crate::ann::layers::layer::{FCLayer, InputLayer};
    use crate::la::matrix::Matrix2D;
    use crate::la::vector::Vector;

    use super::*;

    #[test]
    fn test_feedforward() {
        // Arrange
        let mut model = Model::new();

        let input_layer = InputLayer::new(2);
        model.add(Box::new(input_layer));

        let hidden_layer = FCLayer::new(3, Box::new(Sigmoid {}));
        model.add(Box::new(hidden_layer));

        let output_layer = FCLayer::new(1, Box::new(ReLU {}));
        model.add(Box::new(output_layer));

        let mut mb = model.create_minibatch();
        mb.a[0] = Vector::from(vec![0.0, 1.0]);
        mb.z[0] = Vector::from(vec![0.0, 1.0]);

        // Act
        model.feedforward(&mut mb);

        // Assert
        let weights1 = model.get_weights(1);
        let weights2 = model.get_weights(2);
        let biases1 = model.get_biases(1);
        let biases2 = model.get_biases(2);

        // a^{1}_{0}
        let a10 = activation::sigmoid(weights1[(0, 0)] * mb.a[0][0] + weights1[(0, 1)] * mb.a[0][1] + biases1[0]);
        assert_eq!(a10, mb.a[1][0]);

        // a^{1}_{1}
        let a11 = activation::sigmoid(weights1[(1, 0)] * mb.a[0][0] + weights1[(1, 1)] * mb.a[0][1] + biases1[1]);
        assert_eq!(a11, mb.a[1][1]);

        // a^{1}_{2}
        let a12 = activation::sigmoid(weights1[(2, 0)] * mb.a[0][0] + weights1[(2, 1)] * mb.a[0][1] + biases1[2]);
        assert_eq!(a12, mb.a[1][2]);

        // a^{2}_{0}
        let a20 = activation::relu(weights2[(0, 0)] * mb.a[1][0] + weights2[(0, 1)] * mb.a[1][1] + weights2[(0, 2)] * mb.a[1][2] + biases2[0]);
        assert_eq!(a20, mb.a[2][0]);
    }

    #[test]
    fn test_backprop_errors() {
        // Arrange
        let mut model = Model::new();

        let input_layer = InputLayer::new(2);
        model.add(Box::new(input_layer));

        let hidden_layer = FCLayer::new(2, Box::new(Sigmoid {}));
        model.add(Box::new(hidden_layer));

        let output_layer = FCLayer::new(1, Box::new(Sigmoid {}));
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
        let mut model = Model::new();

        let input_layer = InputLayer::new(2);
        model.add(Box::new(input_layer));

        let hidden_layer = FCLayer::new(2, Box::new(Sigmoid {}));
        model.add(Box::new(hidden_layer));

        let output_layer = FCLayer::new(1, Box::new(Sigmoid {}));
        model.add(Box::new(output_layer));

        let mut mb = model.create_minibatch();
        mb.z[0] = Vector::from(vec![0.0, 1.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);

        // expected output
        let y = Vector::from(vec![0.0]);

        model.feedforward(&mut mb);
        model.backprop(&mut mb, &QuadraticCost {}, &y);

        // Act
        let mbs: [Minibatch; 1] = [mb];
        let (dws, dbs) = model.calculate_derivatives(&mbs);

        // Assert
        assert_eq!(2, dws.len());

        /*
           | dC \ dw^1_00   dC \ dw^1_01 |
           | dC \ dw^1_10   dC \ dw^1_11 |
        */
        assert_eq!(2, dws[0].nrows());
        assert_eq!(2, dws[0].ncols());

        // | dC \ dw^2_00   dC \ dw^2_01 |
        assert_eq!(1, dws[1].nrows());
        assert_eq!(2, dws[1].ncols());

        assert_eq!(2, dbs.len());

        // dC \ db^1_0
        assert_eq!(2, dws[0].nrows());

        // dC \ db^2_0
        // dC \ dw^2_1
        assert_eq!(1, dws[1].nrows());
    }

    #[test]
    fn test_update_network_does_not_throw() {
        // Arrange
        let mut model = Model::new();

        let input_layer = InputLayer::new(2);
        model.add(Box::new(input_layer));

        let hidden_layer = FCLayer::new(2, Box::new(Sigmoid {}));
        model.add(Box::new(hidden_layer));

        let weights2 = Matrix2D::new_from_data(1, 2, vec![0.3, 0.3]);
        let biases2: Vector = vec![0.4].into();
        let output_layer = FCLayer::new(1, Box::new(Sigmoid {}));
        model.add(Box::new(output_layer));

        let mut mb = model.create_minibatch();
        mb.z[0] = Vector::from(vec![0.0, 1.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);

        // expected output
        let y = Vector::from(vec![0.0]);

        model.feedforward(&mut mb);
        model.backprop(&mut mb, &QuadraticCost {}, &y);
        let mbs: [Minibatch; 1] = [mb];
        let (dws, dbs) = model.calculate_derivatives(&mbs);

        // Act
        model.update_network(0.1, dws, dbs);

        // Assert
    }

    #[test]
    fn test_train_model() {
        // Arrange
        let mut model = Model::new();

        let input_layer = InputLayer::new(2);
        model.add(Box::new(input_layer));

        let hidden_layer = FCLayer::new(2, Box::new(Sigmoid {}));
        model.add(Box::new(hidden_layer));

        let output_layer = FCLayer::new(1, Box::new(Sigmoid {}));
        model.add(Box::new(output_layer));

        let mut mb = model.create_minibatch();
        mb.z[0] = Vector::from(vec![0.0, 1.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);

        // expected output
        let y = Vector::from(vec![0.0]);

        //train(&mut self, data: &(&Vec<TrainingData>, &Vec<TrainingData>, &Vec<TrainingData>), epochs: usize, eta: f64, _lambda: f64, minibatch_size: usize, cost_function: &CostFunction) {

        // model an AND gate
        let training_data = vec![
            TrainingData {
                input_activations: Vector::from(vec![0.0, 0.0]),
                output_activations: Vector::from(vec![0.0]),
            },
            TrainingData {
                input_activations: Vector::from(vec![0.0, 1.0]),
                output_activations: Vector::from(vec![0.0]),
            },
            TrainingData {
                input_activations: Vector::from(vec![1.0, 0.0]),
                output_activations: Vector::from(vec![0.0]),
            },
            TrainingData {
                input_activations: Vector::from(vec![1.0, 1.0]),
                output_activations: Vector::from(vec![1.0]),
            },
        ];
        let data = (&training_data, &vec![], &vec![]);

        // Act
        model.train(&data, 1000, 0.05, 1.0, 4, &QuadraticCost {});

        // Assert
        let mut mb = model.create_minibatch();
        mb.z[0] = Vector::from(vec![0.0, 0.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);
        model.feedforward(&mut mb);
        assert_approx_eq!(0.06767588917975884, &mb.a[2][0], 1E-2);

        mb.z[0] = Vector::from(vec![1.0, 0.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);
        model.feedforward(&mut mb);
        //        assert_approx_eq!(0.18941306978303418, &mb.a[2][0], 1E-2);

        mb.z[0] = Vector::from(vec![0.0, 1.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);
        model.feedforward(&mut mb);
        //        assert_approx_eq!(0.06869306322881202, &mb.a[2][0], 1E-2);

        mb.z[0] = Vector::from(vec![1.0, 1.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);
        model.feedforward(&mut mb);
        //        assert_approx_eq!(0.5705021128252346, &mb.a[2][0], 1E-2);
    }

    #[test]
    fn test_train_model2() {
        // Arrange
        let mut model = Model::new();

        let input_layer = InputLayer::new(2);
        model.add(Box::new(input_layer));

        let hidden_layer = FCLayer::new(10, Box::new(Sigmoid {}));
        model.add(Box::new(hidden_layer));

        let output_layer = FCLayer::new(1, Box::new(Sigmoid {}));
        model.add(Box::new(output_layer));

        let mut mb = model.create_minibatch();
        mb.z[0] = Vector::from(vec![0.0, 1.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);

        // expected output
        let y = Vector::from(vec![0.0]);

        // model an AND gate
        let training_data = vec![
            TrainingData {
                input_activations: Vector::from(vec![0.0, 0.0]),
                output_activations: Vector::from(vec![0.0]),
            },
            TrainingData {
                input_activations: Vector::from(vec![0.0, 1.0]),
                output_activations: Vector::from(vec![0.0]),
            },
            TrainingData {
                input_activations: Vector::from(vec![1.0, 0.0]),
                output_activations: Vector::from(vec![0.0]),
            },
            TrainingData {
                input_activations: Vector::from(vec![1.0, 1.0]),
                output_activations: Vector::from(vec![1.0]),
            },
        ];
        let data = (&training_data, &vec![], &vec![]);

        // Act
        model.train(&data, 1000, 0.05, 1.0, 4, &QuadraticCost {});

        // Assert
        let output_layer_index = 2;
        let mut mb = model.create_minibatch();
        mb.z[0] = Vector::from(vec![0.0, 0.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);
        model.feedforward(&mut mb);
        //        assert_approx_eq!(0.04, &mb.a[output_layer_index][0], 1E-2);
        println!("expected: {}   is: {}", 0.0, &mb.a[output_layer_index][0]);

        mb.z[0] = Vector::from(vec![1.0, 0.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);
        model.feedforward(&mut mb);
        //        assert_approx_eq!(0.17, &mb.a[2][0], 1E-2);
        println!("expected: {}   is: {}", 0.0, &mb.a[output_layer_index][0]);

        mb.z[0] = Vector::from(vec![0.0, 1.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);
        model.feedforward(&mut mb);
        //        assert_approx_eq!(0.17, &mb.a[2][0], 1E-2);
        println!("expected: {}   is: {}", 0.0, &mb.a[output_layer_index][0]);

        mb.z[0] = Vector::from(vec![1.0, 1.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);
        model.feedforward(&mut mb);
        //        assert_approx_eq!(0.46, &mb.a[2][0], 1E-2);
        println!("expected: {}   is: {}", 1.0, &mb.a[output_layer_index][0]);
    }

    #[test]
    fn test_train_sin_x() {
        use crate::ann::activation::Sin;

        /* Train f(x) = sin(x) where x is the input activation and
         * sin is the activation function of the output layer.
         * Basically, z^{1}_{0} = sigma(w * x + b), where w=1 and b=0.
         */

        // Arrange
        let mut model = Model::new();

        let input_layer = InputLayer::new(1);
        model.add(Box::new(input_layer));

        let output_layer = FCLayer::new(1, Box::new(Sin {}));
        model.add(Box::new(output_layer));

        // SS: restrict input to (-pi/2, pi/2) because of periodicity
        let ntraining_samples = 1000;
        let step = std::f64::consts::PI / ntraining_samples as f64;
        let training_data = (0..ntraining_samples)
            .map(|x| ((x as f64 - ntraining_samples as f64 / 2.0) * step))
            .map(|x| TrainingData {
                input_activations: Vector::from(vec![x]),
                output_activations: Vector::from(vec![x.sin()]),
            })
            .collect::<Vec<_>>();
        let data = (&training_data, &vec![], &vec![]);

        // Act
        model.train(&data, 10, 0.05, 1.0, 4, &QuadraticCost {});

        // Assert
        let weights = model.get_weights(1);
        let expected_weight = 1.0;
        assert_approx_eq!(expected_weight, weights[(0, 0)], 1E-8);

        let biases = model.get_biases(1);
        let expected_bias = 0.0;
        assert_approx_eq!(expected_bias, biases[0], 1E-8);
    }

    #[test]
    fn test_train_sin_x_plus_sin_y() {
        use crate::ann::activation::{Id, Sin};

        /* Train f(x) = u1 * sin(x) + u2 * sin(x) + b2 where x is the input activation and
         * sin is the activation function of the hidden layer. Id is the activation function
         * for the output layer.
         * Basically, z^{2}_{0} = sigma2(w2_0 * sigma1(x) + w2_1 * sigma1(x)) + b2, where w=1 and b=0.
         */

        // Arrange
        let mut model = Model::new();

        let input_layer = InputLayer::new(1);
        model.add(Box::new(input_layer));

        let hidden_layer = FCLayer::new(2, Box::new(Sin {}));
        model.add(Box::new(hidden_layer));

        let output_layer = FCLayer::new(1, Box::new(Id {}));
        model.add(Box::new(output_layer));

        // SS: restrict input to (-pi/2, pi/2) because of periodicity
        let w2_0 = 1.2;
        let w2_1 = 0.87;
        let b2 = -1.1;
        let ntraining_samples = 1000;
        let step = std::f64::consts::PI / ntraining_samples as f64;
        let training_data = (0..ntraining_samples)
            .map(|x| ((x as f64 - ntraining_samples as f64 / 2.0) * step))
            .map(|x| TrainingData {
                input_activations: Vector::from(vec![x]),
                output_activations: Vector::from(vec![w2_0 * x.sin() + w2_1 * x.sin() + b2]),
            })
            .collect::<Vec<_>>();
        let data = (&training_data, &vec![], &vec![]);

        // Act
        model.train(&data, 1000, 0.05, 1.0, 25, &QuadraticCost {});

        // Assert
        let weights = model.get_weights(1);
        let weights = model.get_weights(2);
        let expected_weight = 1.0;
        //        assert_approx_eq!(expected_weight, weights[(0, 0)], 1E-8);

        let biases = model.get_biases(1);
        let biases = model.get_biases(2);
        let expected_bias = 0.0;
        //        assert_approx_eq!(expected_bias, biases[0], 1E-8);

        let output_layer_index = 2;
        let mut mb = model.create_minibatch();
        mb.a[0] = Vector::from(vec![1.5299556222982293]);
        model.feedforward(&mut mb);
        //        assert_approx_eq!(0.46, &mb.a[2][0], 1E-2);
        println!("expected: {}   is: {}", 1.0, &mb.a[output_layer_index][0]);
    }
}

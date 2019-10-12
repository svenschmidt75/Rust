#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::cmp;

use rand::seq::SliceRandom;
use rand::thread_rng;

use linear_algebra::matrix::Matrix2D;
use linear_algebra::ops;
use linear_algebra::vector::Vector;

use crate::ann::activation::Activation;
use crate::ann::cost_function::CostFunction;
use crate::ann::cost_function::QuadraticCost;
use crate::ann::layers::activation_layer::ActivationLayer;
use crate::ann::layers::dropout_layer::DropoutLayer;
use crate::ann::layers::fc_layer::FCLayer;
use crate::ann::layers::input_layer::InputLayer;
use crate::ann::layers::layer::Layer;
use crate::ann::layers::softmax_layer::SoftMaxLayer;
use crate::ann::minibatch::Minibatch;
use crate::ann::training_data::TrainingData;

#[derive(Default)]
pub struct Model {
    layers: Vec<Layer>,
}

fn get_neighbors<T>(v: &mut Vec<T>, idx: usize) -> (&T, &mut T) {
    let (l, r) = v.split_at_mut(idx);
    let item1 = l.last().unwrap();
    let item2 = &mut r[0];
    (item1, item2)
}

impl Model {
    pub fn new() -> Model {
        Model { layers: vec![] }
    }

    pub fn addInputLayer(&mut self, layer: InputLayer) {
        self.layers.push(Layer::from(layer))
    }

    pub fn addFullyConnectedLayer(&mut self, layer: FCLayer) {
        self.layers.push(Layer::from(layer))
    }

    pub fn addDropoutLayer(&mut self, layer: DropoutLayer) {
        self.layers.push(Layer::from(layer))
    }

    pub fn addActivationLayer(&mut self, layer: ActivationLayer) {
        self.layers.push(Layer::from(layer))
    }

    pub fn addSoftMaxLayer(&mut self, layer: SoftMaxLayer) {
        self.layers.push(Layer::from(layer))
    }

    pub fn get_layer(&self, idx: usize) -> &Layer {
        &self.layers[idx]
    }

    pub fn output_layer_index(&self) -> usize {
        self.layers.len() - 1
    }

    fn number_of_minibatches(training_data_size: usize, minibatch_size: usize) -> (usize, usize) {
        let n_minibatches = training_data_size / minibatch_size;
        (n_minibatches, training_data_size % minibatch_size)
    }

    pub fn train(&mut self, data: &(&[TrainingData], &[TrainingData], &[TrainingData]), epochs: usize, eta: f64, rho: f64, lambda: f64, minibatch_size: usize, cost_function: &dyn CostFunction) {
        // call initialize on each layer
        self.initialize_layers();

        let training_data = data.0;
        let mut trainingdata_indices: Vec<_> = (0..training_data.len()).collect();
        let mut rng = thread_rng();

        // SS: training samples per minibatch
        let mb_size = cmp::min(training_data.len(), minibatch_size);
        let mut mbs = (0..mb_size).map(|_| self.create_minibatch()).collect::<Vec<_>>();

        let (n_minibatches, _remainder) = Model::number_of_minibatches(trainingdata_indices.len(), mb_size);

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
        println!("L2 regularization: {}", lambda);
        println!("Momentum: {}", rho);
        println!();

        for epoch in 0..epochs {
            // random shuffle on training data
            trainingdata_indices.shuffle(&mut rng);

            // SS: Split the training data into chunks, each the size of a minibatch.
            // #chunks = mb_size, plus remainder
            let chunks = trainingdata_indices.chunks(mb_size);

            for (_chunk_index, chunk) in chunks.enumerate() {
                let mut known_classifications = Vec::with_capacity(chunk.len());
                // SS: insert training data into minibatches
                for idx in 0..chunk.len() {
                    let mb = &mut mbs[idx];
                    let training_sample_idx = chunk[idx];
                    let training_sample = &training_data[training_sample_idx];
                    known_classifications.push(&training_sample.output_activations);
                    mb.output[0] = training_sample.input_activations.clone();
                }
                self.feedforward(&mut mbs);
                self.backprop(&mut mbs, &known_classifications, cost_function);
                self.update_network(&mbs, eta, rho, lambda);
            }

            // SS: epoch completed, print cost for all training samples, print accuracy
            let cost = cost_function.cost(self, training_data, lambda);
            let error = QuadraticCost {}.cost(self, training_data, lambda);
            let train_accuracy = self.accuracy(training_data);
            let test_accuracy = self.accuracy(data.2);
            println!("Epoch {} - cost {} - error {} - train acc {} - test acc {}", epoch + 1, cost, error, train_accuracy, test_accuracy);
        }
    }

    fn accuracy(&mut self, test_data: &[TrainingData]) -> f64 {
        let accuracy;
        let mut same = 0;
        let mut mb = self.create_minibatch();
        let output_layer_index = self.output_layer_index();
        for x in test_data {
            mb.output[0] = x.input_activations.clone();
            self.feedforward_minibatch(&mut mb);
            let output_activations = &mb.output[output_layer_index];
            let expected_output_layer_activations = &x.output_activations;
            let is_classification = Model::get_class(output_activations);
            let expected_class = Model::get_class(expected_output_layer_activations);
            if expected_class == is_classification {
                same += 1;
            }
        }
        accuracy = f64::from(same) / test_data.len() as f64;
        accuracy
    }

    fn get_class(data: &Vector) -> usize {
        let mut index = 0;
        let mut value = 0.0;
        for (idx, v) in data.iter().enumerate() {
            if *v > value {
                value = *v;
                index = idx;
            }
        }
        index
    }

    fn next_training_sample(&mut self) {
        self.layers.iter_mut().for_each(|layer| layer.new_feedforward());
    }

    fn next_minibatch(&mut self, mbs: &[Minibatch]) {
        self.layers.iter_mut().for_each(|layer| layer.new_minibatch(mbs));
    }

    fn initialize_layers(&mut self) {
        for idx in 1..self.layers.len() {
            let (prev, current) = get_neighbors(&mut self.layers, idx);
            current.initialize(prev);
        }
    }

    pub fn create_minibatch(&self) -> Minibatch {
        let mut nas: Vec<_> = self.layers.iter().map(|layer| layer.number_of_neurons()).collect();

        // SS: add one "hidden" layer
        // We are only going to use its error property,
        // which gets initialized with the initial
        // // error of the cost function, i.e. dC\dA_L.
        nas.push(self.layers.iter().last().unwrap().number_of_neurons());
        Minibatch::new(nas)
    }

    pub fn feedforward(&mut self, mbs: &mut [Minibatch]) {
        // SS: feed forward all minibatch items for one entire layer, before
        // advancing to the next layer.
        self.layers.iter_mut().enumerate().skip(1).for_each(|(layer_index, layer)| {
            layer.new_minibatch(mbs);
            for i in 0..mbs.len() {
                layer.new_feedforward();
                let mb = &mut mbs[i];
                layer.feedforward(layer_index, mb);
            }
        });
    }

    pub fn feedforward_minibatch(&self, mb: &mut Minibatch) {
        // SS: feed forward one instance of a training data sample
        // and record all calculated activations for all layers
        // for backprop.
        self.layers.iter().enumerate().skip(1).for_each(|(layer_index, layer)| layer.feedforward(layer_index, mb));
    }

    fn calculate_outputlayer_error(&self, mb: &mut Minibatch, y: &Vector, cost_function: &dyn CostFunction) {
        let output_layer_index = self.output_layer_index();
        let aL = &mb.output[output_layer_index];

        // SS: calculate dC/da^{L}
        let dCda = cost_function.output_error(aL, y);

        mb.error[output_layer_index + 1] = dCda;
    }

    fn backprop(&self, mbs: &mut [Minibatch], y: &[&Vector], cost_function: &dyn CostFunction) {
        let output_layer_index = self.output_layer_index();
        for layer_index in (1..=output_layer_index).rev() {
            let layer = &self.layers[layer_index];
            for i in 0..mbs.len() {
                let mb = &mut mbs[i];
                self.calculate_outputlayer_error(mb, y[i], cost_function);
                layer.backprop(layer_index, mb);
            }
        }
    }

    fn backprop_minibatch(&self, mb: &mut Minibatch, y: &Vector, cost_function: &dyn CostFunction) {
        let output_layer_index = self.output_layer_index();
        self.calculate_outputlayer_error(mb, y, cost_function);
        for layer_index in (1..=output_layer_index).rev() {
            let layer = &self.layers[layer_index];
            layer.backprop(layer_index, mb);
        }
    }

    fn update_network(&mut self, mbs: &[Minibatch], eta: f64, rho: f64, lambda: f64) {
        let output_layer_index = self.output_layer_index();
        for layer_index in 1..=output_layer_index {
            let current_layer = &mut self.layers[layer_index];
            current_layer.update_network(layer_index, mbs, eta, rho, lambda);
        }
    }

    pub fn summary(&self) {}

    pub(crate) fn weights_squared_sum(&self) -> f64 {
        // SS: skip  the input layer
        self.layers.iter().skip(1).fold(0.0, |accum, layer| accum + layer.weights_squared_sum())
    }

    pub(crate) fn get_weights(&self, layer_index: usize) -> &Matrix2D {
        match &self.layers[layer_index] {
            Layer::FullyConnected(layer) => layer.get_weights(),
            _ => panic!("get_weights: layer does not have weights"),
        }
    }

    pub(crate) fn set_weights(&mut self, layer_index: usize, weights: Matrix2D) {
        match &mut self.layers[layer_index] {
            Layer::FullyConnected(layer) => layer.set_weights(weights),
            _ => panic!("get_weights: layer does not have weights"),
        }
    }

    pub(crate) fn get_biases(&self, layer_index: usize) -> &Vector {
        match &self.layers[layer_index] {
            Layer::FullyConnected(layer) => layer.get_biases(),
            _ => panic!("get_weights: layer does not have biases"),
        }
    }

    pub(crate) fn set_biases(&mut self, layer_index: usize, biases: Vector) {
        match &mut self.layers[layer_index] {
            Layer::FullyConnected(layer) => layer.set_biases(biases),
            _ => panic!("get_weights: layer does not have biases"),
        }
    }

    fn numerical_derivative_bias(&mut self, training_samples: &[TrainingData], layer_index: usize, la: usize, cost: &dyn CostFunction, lambda: f64) -> f64 {
        // SS: numerically calculate b^{layer_index}_{la}, where la is the neuron index in layer_index.
        let delta = 0.000_001;

        let mut biases = self.get_biases(layer_index).clone();
        let b = biases[la];
        biases[la] = b - delta;
        self.set_biases(layer_index, biases);
        let c1 = cost.cost(self, training_samples, lambda);

        let mut biases = self.get_biases(layer_index).clone();
        biases[la] = b + delta;
        self.set_biases(layer_index, biases);
        let c2 = cost.cost(self, training_samples, lambda);

        // important, restore original bias
        let mut biases = self.get_biases(layer_index).clone();
        biases[la] = b;
        self.set_biases(layer_index, biases);

        let dc = (c2 - c1) / delta / 2_f64;
        dc
    }

    fn grad_bias(&mut self, layer_index: usize, xs: &[TrainingData], cost: &dyn CostFunction, lambda: f64) -> Vector {
        // SS: same as calculate_derivatives, but here we are using recursion
        assert!(layer_index > 0);
        let layer = self.get_layer(layer_index);
        let fc_layer = if let Layer::FullyConnected(l) = layer { l } else { panic!("not a fully-connected layer") };

        let mut mbs = vec![self.create_minibatch()];
        let mut db = Vector::new(layer.number_of_neurons());

        for training_sample in xs {
            let y = &training_sample.output_activations;
            let mb = &mut mbs[0];
            mb.output[0] = training_sample.input_activations.clone();
            self.feedforward_minibatch(mb);
            self.backprop_minibatch(mb, y, cost);
            let (_, db2) = fc_layer.calculate_derivatives(layer_index, &mbs[..], lambda);
            db += &db2;
        }
        db /= xs.len();
        db
    }

    fn numerical_derivative_weight(&mut self, training_samples: &[TrainingData], layer_index: usize, la: usize, pa: usize, cost: &dyn CostFunction, lambda: f64) -> f64 {
        // SS: numerically calculate w^{layer_index}_{la, pa}, where la is the neuron index in layer_index
        // and pa is the neuron index in the previous layer.
        let delta = 0.000_001;

        let mut weights = self.get_weights(layer_index).clone();
        let w = weights[(la, pa)];
        weights[(la, pa)] = w - delta;
        self.set_weights(layer_index, weights);
        let c1 = cost.cost(self, training_samples, lambda);

        let mut weights = self.get_weights(layer_index).clone();
        weights[(la, pa)] = w + delta;
        self.set_weights(layer_index, weights);
        let c2 = cost.cost(self, training_samples, lambda);

        // important, restore original weight
        let mut weights = self.get_weights(layer_index).clone();
        weights[(la, pa)] = w;
        self.set_weights(layer_index, weights);

        let dc = (c2 - c1) / delta / 2_f64;
        dc
    }

    fn grad_weight(&mut self, layer_index: usize, xs: &[TrainingData], cost: &dyn CostFunction, lambda: f64) -> Matrix2D {
        // SS: same as calculate_derivatives, but here we are using recursion
        assert!(layer_index > 0);
        let prev_layer = self.get_layer(layer_index - 1);
        let layer = self.get_layer(layer_index);
        let fc_layer = if let Layer::FullyConnected(l) = layer { l } else { panic!("not a fully-connected layer") };

        let mut mbs = vec![self.create_minibatch()];
        let mut dw = Matrix2D::new(layer.number_of_neurons(), prev_layer.number_of_neurons());

        for training_sample in xs {
            let y = &training_sample.output_activations;
            let mb = &mut mbs[0];
            mb.output[0] = training_sample.input_activations.clone();
            self.feedforward_minibatch(mb);
            self.backprop_minibatch(mb, y, cost);
            let (dw2, _) = fc_layer.calculate_derivatives(layer_index, &mbs[..], lambda);
            dw += &dw2;
        }
        dw /= xs.len();
        dw
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use assert_approx_eq::assert_approx_eq;
    use linear_algebra::matrix::Matrix2D;
    use linear_algebra::vector::Vector;
    use mnist_loader::labels::Label;
    use mnist_loader::loader::{load_image_file, load_label_file};

    use crate::ann::activation;
    use crate::ann::activation::ReLU;
    use crate::ann::activation::Sigmoid;
    use crate::ann::cost_function::{CrossEntropyCost, QuadraticCost};
    use crate::ann::layers::activation_layer::ActivationLayer;
    use crate::ann::layers::{fc_layer::FCLayer, input_layer::InputLayer};

    use super::*;

    #[test]
    fn test_analytical_equals_numerical_derivative_1() {
        use crate::ann::activation::Id;

        // Model used: f(x) = a * x + b * x + c
        // w^{1}_{00} = a = 1.8
        // w^{1}_{10} = b = 0.5
        // Activation function  for hidden layer: Id
        // w^{2}_{00} = 1
        // w^{2}_{01} = 1
        // Activation function  for output layer: Id
        // b^{1}_{0} + b^{1}_{1} + b^{2}_{0} = c = -1.2

        // Arrange
        let cost_function = QuadraticCost;

        let mut model = Model::new();
        model.addInputLayer(InputLayer::new(1));
        model.addFullyConnectedLayer(FCLayer::new(2));
        model.addActivationLayer(ActivationLayer::new(2, Box::new(Id {})));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(Id {})));

        // SS: restrict input to (-pi/2, pi/2) because of periodicity
        let u1 = 1.8;
        let u2 = 0.5;
        let c = -1.2;
        let ntraining_samples = 1000;
        let step = std::f64::consts::PI / ntraining_samples as f64;
        let training_data = (0..ntraining_samples)
            .map(|x| ((x as f64 - ntraining_samples as f64 / 2.0) * step))
            .map(|x| TrainingData {
                input_activations: Vector::from(vec![x]),
                output_activations: Vector::from(vec![u1 * x + u2 * x + c]),
            })
            .collect::<Vec<_>>();
        let tmp: [TrainingData; 0] = [];
        let data = (&training_data[..], &tmp as &[TrainingData], &tmp as &[TrainingData]);
        model.train(&data, 5, 0.005, 0.0, 0.0, 25, &cost_function);

        // Act
        // Assert

        // layer 3 - fully-connected layer
        let db_numeric = model.numerical_derivative_bias(&training_data[..], 3, 0, &cost_function, 0.0);
        let db_analytic = model.grad_bias(3, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(db_numeric, db_analytic[0], 1E-6);

        let dw_numeric_1 = model.numerical_derivative_weight(&training_data[..], 3, 0, 0, &cost_function, 0.0);
        let dw_numeric_2 = model.numerical_derivative_weight(&training_data[..], 3, 0, 1, &cost_function, 0.0);
        let dw_analytic = model.grad_weight(3, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(dw_numeric_1, dw_analytic[(0, 0)], 1E-8);
        assert_approx_eq!(dw_numeric_2, dw_analytic[(0, 1)], 1E-8);

        // layer 1 - fully-connected layer
        let db_numeric_1 = model.numerical_derivative_bias(&training_data[..], 1, 0, &cost_function, 0.0);
        let db_numeric_2 = model.numerical_derivative_bias(&training_data[..], 1, 1, &cost_function, 0.0);
        let db_analytic = model.grad_bias(1, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(db_numeric_1, db_analytic[0], 1E-8);
        assert_approx_eq!(db_numeric_2, db_analytic[1], 1E-8);

        let dw_numeric_1 = model.numerical_derivative_weight(&training_data[..], 1, 0, 0, &cost_function, 0.0);
        let dw_numeric_2 = model.numerical_derivative_weight(&training_data[..], 1, 1, 0, &cost_function, 0.0);
        let dw_analytic = model.grad_weight(1, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(dw_numeric_1, dw_analytic[(0, 0)], 1E-8);
        assert_approx_eq!(dw_numeric_2, dw_analytic[(1, 0)], 1E-8);
    }

    #[test]
    fn test_analytical_equals_numerical_derivative_2() {
        use crate::ann::activation::{Id, Sin};

        /* Train f(x) = u1 * sin(u0 * x + b0) + b1 where x is the input activation and
         * sin is the activation function of the hidden layer. Id is the activation function
         * for the output layer.
         */

        // Arrange
        let cost_function = QuadraticCost;

        let mut model = Model::new();
        model.addInputLayer(InputLayer::new(1));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(Sin {})));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(Id {})));

        // SS: restrict input to (-pi/2, pi/2) because of periodicity
        let w1_0 = 2.345;
        let w2_0 = -8.354;
        let b1 = -0.63;
        let b2 = 1.932;
        let ntraining_samples = 1000;
        let step = std::f64::consts::PI / ntraining_samples as f64;
        let training_data = (0..ntraining_samples)
            .map(|x| ((x as f64 - ntraining_samples as f64 / 2.0) * step))
            .map(|x| TrainingData {
                input_activations: Vector::from(vec![x]),
                output_activations: Vector::from(vec![w2_0 * (w1_0 * x + b1).sin() + b2]),
            })
            .collect::<Vec<_>>();
        let tmp: [TrainingData; 0] = [];
        let data = (&training_data[..], &tmp as &[TrainingData], &tmp as &[TrainingData]);
        model.train(&data, 10, 0.05, 0.0, 1.0, 25, &cost_function);

        // Act
        let mut mb = model.create_minibatch();
        let training_sample = &training_data[0];
        mb.output[0] = training_sample.input_activations.clone();
        model.feedforward_minibatch(&mut mb);

        // Assert

        // layer 3 - fully-connected layer
        let db_numeric = model.numerical_derivative_bias(&training_data[..], 3, 0, &cost_function, 0.0);
        let db_analytic = model.grad_bias(3, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(db_numeric, db_analytic[0], 1E-6);

        let dw_numeric = model.numerical_derivative_weight(&training_data[..], 3, 0, 0, &cost_function, 0.0);
        let dw_analytic = model.grad_weight(3, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(dw_numeric, dw_analytic[(0, 0)], 1E-6);

        // layer 1 - fully-connected layer
        let db_numeric = model.numerical_derivative_bias(&training_data[..], 1, 0, &cost_function, 0.0);
        let db_analytic = model.grad_bias(1, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(db_numeric, db_analytic[0], 1E-4);

        let dw_numeric = model.numerical_derivative_weight(&training_data[..], 1, 0, 0, &cost_function, 0.0);
        let dw_analytic = model.grad_weight(1, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(dw_numeric, dw_analytic[(0, 0)], 1E-4);
    }

    #[test]
    fn test_analytical_equals_numerical_derivative_3() {
        /* Train f(x) = u1 * sin(u0 * x + b0) + b1 where x is the input activation and
         * sin is the activation function of the hidden layer. Id is the activation function
         * for the output layer.
         */

        // Arrange
        let cost_function = QuadraticCost;

        let mut model = Model::new();
        model.addInputLayer(InputLayer::new(1));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(Sigmoid {})));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(Sigmoid {})));

        // SS: restrict input to (-pi/2, pi/2) because of periodicity
        let w1_0 = 2.345;
        let w2_0 = -8.354;
        let b1 = -0.63;
        let b2 = 1.932;
        let ntraining_samples = 1000;
        let step = std::f64::consts::PI / ntraining_samples as f64;
        let training_data = (0..ntraining_samples)
            .map(|x| ((x as f64 - ntraining_samples as f64 / 2.0) * step))
            .map(|x| TrainingData {
                input_activations: Vector::from(vec![x]),
                output_activations: Vector::from(vec![activation::sigmoid(w2_0 * activation::sigmoid(w1_0 * x + b1) + b2)]),
            })
            .collect::<Vec<_>>();
        let tmp: [TrainingData; 0] = [];
        let data = (&training_data[..], &tmp as &[TrainingData], &tmp as &[TrainingData]);
        model.train(&data, 10, 0.05, 0.0, 0.0, 25, &cost_function);

        // Act
        let mut mb = model.create_minibatch();
        let training_sample = &training_data[0];
        mb.output[0] = training_sample.input_activations.clone();
        model.feedforward_minibatch(&mut mb);

        // Assert

        // layer 3 - fully-connected layer
        let db_numeric = model.numerical_derivative_bias(&training_data[..], 3, 0, &cost_function, 0.0);
        let db_analytic = model.grad_bias(3, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(db_numeric, db_analytic[0], 1E-6);

        let dw_numeric = model.numerical_derivative_weight(&training_data[..], 3, 0, 0, &cost_function, 0.0);
        let dw_analytic = model.grad_weight(3, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(dw_numeric, dw_analytic[(0, 0)], 1E-6);

        // layer 1 - fully-connected layer
        let db_numeric = model.numerical_derivative_bias(&training_data[..], 1, 0, &cost_function, 0.0);
        let db_analytic = model.grad_bias(1, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(db_numeric, db_analytic[0], 1E-4);

        let dw_numeric = model.numerical_derivative_weight(&training_data[..], 1, 0, 0, &cost_function, 0.0);
        let dw_analytic = model.grad_weight(1, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(dw_numeric, dw_analytic[(0, 0)], 1E-4);
    }

    #[test]
    fn test_derivative_3_with_l2_regularization() {
        /* Train f(x) = u1 * sin(u0 * x + b0) + b1 where x is the input activation and
         * sin is the activation function of the hidden layer. Id is the activation function
         * for the output layer.
         */

        // Arrange
        let cost_function = QuadraticCost;

        let mut model = Model::new();
        model.addInputLayer(InputLayer::new(1));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(Sigmoid {})));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(Sigmoid {})));

        // SS: restrict input to (-pi/2, pi/2) because of periodicity
        let w1_0 = 2.345;
        let w2_0 = -8.354;
        let b1 = -0.63;
        let b2 = 1.932;
        let ntraining_samples = 1000;
        let step = std::f64::consts::PI / ntraining_samples as f64;
        let training_data = (0..ntraining_samples)
            .map(|x| ((x as f64 - ntraining_samples as f64 / 2.0) * step))
            .map(|x| TrainingData {
                input_activations: Vector::from(vec![x]),
                output_activations: Vector::from(vec![activation::sigmoid(w2_0 * activation::sigmoid(w1_0 * x + b1) + b2)]),
            })
            .collect::<Vec<_>>();
        let tmp: [TrainingData; 0] = [];
        let data = (&training_data[..], &tmp as &[TrainingData], &tmp as &[TrainingData]);
        model.train(&data, 10, 0.05, 0.0, 1.0, 25, &cost_function);

        // Act
        let mut mb = model.create_minibatch();
        let training_sample = &training_data[0];
        mb.output[0] = training_sample.input_activations.clone();
        model.feedforward_minibatch(&mut mb);

        // Assert

        // layer 3 - fully-connected layer
        let db_numeric = model.numerical_derivative_bias(&training_data[..], 3, 0, &cost_function, 0.0);
        let db_analytic = model.grad_bias(3, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(db_numeric, db_analytic[0], 1E-6);

        let dw_numeric = model.numerical_derivative_weight(&training_data[..], 3, 0, 0, &cost_function, 0.0);
        let dw_analytic = model.grad_weight(3, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(dw_numeric, dw_analytic[(0, 0)], 1E-6);

        // layer 1 - fully-connected layer
        let db_numeric = model.numerical_derivative_bias(&training_data[..], 1, 0, &cost_function, 0.0);
        let db_analytic = model.grad_bias(1, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(db_numeric, db_analytic[0], 1E-4);

        let dw_numeric = model.numerical_derivative_weight(&training_data[..], 1, 0, 0, &cost_function, 0.0);
        let dw_analytic = model.grad_weight(1, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(dw_numeric, dw_analytic[(0, 0)], 1E-4);
    }

    #[test]
    fn test_derivative_crossentropy_1() {
        /* Train f(x) = u1 * sin(u0 * x + b0) + b1 where x is the input activation and
         * sin is the activation function of the hidden layer. Id is the activation function
         * for the output layer.
         */

        // Arrange
        let cost_function = CrossEntropyCost;

        let mut model = Model::new();
        model.addInputLayer(InputLayer::new(2));
        model.addFullyConnectedLayer(FCLayer::new(5));
        model.addActivationLayer(ActivationLayer::new(5, Box::new(Sigmoid {})));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(Sigmoid {})));

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
        let tmp: [TrainingData; 0] = [];
        let data = (&training_data[..], &tmp as &[TrainingData], &tmp as &[TrainingData]);

        // Act
        model.train(&data, 100, 5.0, 0.0, 1.0, 4, &cost_function);

        // Assert

        // layer 3 - fully-connected layer
        let db_numeric = model.numerical_derivative_bias(&training_data[..], 3, 0, &cost_function, 0.0);
        let db_analytic = model.grad_bias(3, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(db_numeric, db_analytic[0], 1E-6);

        let dw_numeric = model.numerical_derivative_weight(&training_data[..], 3, 0, 0, &cost_function, 0.0);
        let dw_analytic = model.grad_weight(3, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(dw_numeric, dw_analytic[(0, 0)], 1E-6);

        // layer 1 - fully-connected layer
        let db_numeric = model.numerical_derivative_bias(&training_data[..], 1, 0, &cost_function, 0.0);
        let db_analytic = model.grad_bias(1, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(db_numeric, db_analytic[0], 1E-4);

        let dw_numeric = model.numerical_derivative_weight(&training_data[..], 1, 0, 0, &cost_function, 0.0);
        let dw_analytic = model.grad_weight(1, &training_data[..], &cost_function, 0.0);
        assert_approx_eq!(dw_numeric, dw_analytic[(0, 0)], 1E-4);
    }

    #[test]
    fn test_feedforward() {
        // Arrange
        let mut model = Model::new();
        model.addInputLayer(InputLayer::new(2));
        model.addFullyConnectedLayer(FCLayer::new(3));
        model.addActivationLayer(ActivationLayer::new(3, Box::new(Sigmoid {})));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(ReLU {})));

        let mut mb = model.create_minibatch();
        mb.output[0] = Vector::from(vec![0.0, 1.0]);

        model.initialize_layers();

        // Act
        model.feedforward_minibatch(&mut mb);

        // Assert
        let weights1 = model.get_weights(1);
        let weights2 = model.get_weights(3);
        let biases1 = model.get_biases(1);
        let biases2 = model.get_biases(3);

        // a^{2}_{0}
        let a10 = activation::sigmoid(weights1[(0, 0)] * mb.output[0][0] + weights1[(0, 1)] * mb.output[0][1] + biases1[0]);
        assert_eq!(a10, mb.output[2][0]);

        // a^{2}_{1}
        let a11 = activation::sigmoid(weights1[(1, 0)] * mb.output[0][0] + weights1[(1, 1)] * mb.output[0][1] + biases1[1]);
        assert_eq!(a11, mb.output[2][1]);

        // a^{2}_{2}
        let a12 = activation::sigmoid(weights1[(2, 0)] * mb.output[0][0] + weights1[(2, 1)] * mb.output[0][1] + biases1[2]);
        assert_eq!(a12, mb.output[2][2]);

        // a^{2}_{0}
        let a20 = activation::relu(weights2[(0, 0)] * mb.output[2][0] + weights2[(0, 1)] * mb.output[2][1] + weights2[(0, 2)] * mb.output[2][2] + biases2[0]);
        assert_eq!(a20, mb.output[4][0]);
    }

    #[test]
    fn test_train_model_l2_regularization() {
        // Arrange

        let cost_function = CrossEntropyCost;

        let mut model = Model::new();
        model.addInputLayer(InputLayer::new(2));
        model.addFullyConnectedLayer(FCLayer::new(10));
        model.addActivationLayer(ActivationLayer::new(10, Box::new(Sigmoid {})));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(Sigmoid {})));

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
        let tmp: [TrainingData; 0] = [];
        let data = (&training_data[..], &tmp as &[TrainingData], &tmp as &[TrainingData]);

        // Act
        model.train(&data, 1000, 7.0, 0.0, 0.000001, 4, &cost_function);

        // Assert
        let output_layer_index = 4;
        let mut mb = model.create_minibatch();
        mb.output[0] = Vector::from(vec![0.0, 0.0]);
        model.feedforward_minibatch(&mut mb);
        assert_approx_eq!(0.000000008600374481948007, &mb.output[output_layer_index][0], 1E-6);

        mb.output[0] = Vector::from(vec![1.0, 0.0]);
        model.feedforward_minibatch(&mut mb);
        assert_approx_eq!(0.0002504695377738481, &mb.output[output_layer_index][0], 1E-3);

        mb.output[0] = Vector::from(vec![0.0, 1.0]);
        model.feedforward_minibatch(&mut mb);
        assert_approx_eq!(0.00023494173889617028, &mb.output[output_layer_index][0], 1E-3);

        mb.output[0] = Vector::from(vec![1.0, 1.0]);
        model.feedforward_minibatch(&mut mb);
        assert_approx_eq!(0.9992958721912137, &mb.output[output_layer_index][0], 1E-3);
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
        model.addInputLayer(InputLayer::new(1));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(Sin {})));

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
        let tmp: [TrainingData; 0] = [];
        let data = (&training_data[..], &tmp as &[TrainingData], &tmp as &[TrainingData]);

        // Act
        model.train(&data, 10, 0.05, 0.0, 0.0, 4, &QuadraticCost {});

        // Assert
        let weights = model.get_weights(1);
        let expected_weight = 1.0;
        assert_approx_eq!(expected_weight, weights[(0, 0)], 1E-8);

        let biases = model.get_biases(1);
        let expected_bias = 0.0;
        assert_approx_eq!(expected_bias, biases[0], 1E-8);
    }

    #[test]
    fn test_train_4() {
        use crate::ann::activation::Id;

        /* Train f(x) = u1 * x + u2 * x + c where x is the input activation and
         * Id are the activation functions of the hidden layer and the output layer.
         * Note: The solution is
         * u1 + u2 = w^{2}_{0, 0} * w^{1}_{0, 0} + w^{2}_{0, 1} * w^{1}_{1, 0}
         * c = w^{2}_{0, 0} * b^{1}_{0} + w^{2}_{0, 1} * b^{1}_{1} + b^{2}_{0}
         */

        // Arrange
        let mut model = Model::new();
        model.addInputLayer(InputLayer::new(1));
        model.addFullyConnectedLayer(FCLayer::new(2));
        model.addActivationLayer(ActivationLayer::new(2, Box::new(Id {})));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(Id {})));

        // SS: restrict input to (-pi/2, pi/2) because of periodicity
        let u1 = 1.8;
        let u2 = 0.5;
        let c = -1.2;
        let ntraining_samples = 1000;
        let step = std::f64::consts::PI / ntraining_samples as f64;
        let training_data = (0..ntraining_samples)
            .map(|x| ((x as f64 - ntraining_samples as f64 / 2.0) * step))
            .map(|x| TrainingData {
                input_activations: Vector::from(vec![x]),
                output_activations: Vector::from(vec![u1 * x + u2 * x + c]),
            })
            .collect::<Vec<_>>();
        let tmp: [TrainingData; 0] = [];
        let data = (&training_data[..], &tmp as &[TrainingData], &tmp as &[TrainingData]);

        // Act
        model.train(&data, 10, 0.05, 0.0, 0.0, 25, &QuadraticCost {});

        // Assert
        let b1 = model.get_biases(1);
        let b2 = model.get_biases(3);
        let w1 = model.get_weights(1);
        let w2 = model.get_weights(3);
        assert_approx_eq!(u1 + u2, w2[(0, 0)] * w1[(0, 0)] + w2[(0, 1)] * w1[(1, 0)], 1E-8);
        assert_approx_eq!(c, w2[(0, 0)] * b1[0] + w2[(0, 1)] * b1[1] + b2[0], 1E-8);
    }
}

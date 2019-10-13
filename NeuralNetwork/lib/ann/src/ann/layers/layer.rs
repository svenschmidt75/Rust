#![allow(dead_code)]

use crate::ann::layers::activation_layer::ActivationLayer;
use crate::ann::layers::batch_normalize::BatchNormalizeLayer;
use crate::ann::layers::dropout_layer::DropoutLayer;
use crate::ann::layers::fc_layer::FCLayer;
use crate::ann::layers::input_layer::InputLayer;
use crate::ann::layers::layer::Layer::FullyConnected;
use crate::ann::layers::softmax_layer::SoftMaxLayer;
use crate::ann::minibatch::Minibatch;

pub enum Layer {
    Input(InputLayer),
    FullyConnected(FCLayer),
    Dropout(DropoutLayer),
    Activation(ActivationLayer),
    SoftMax(SoftMaxLayer),
    BatchNormalize(BatchNormalizeLayer),
}

impl From<InputLayer> for Layer {
    fn from(l: InputLayer) -> Self {
        Layer::Input(l)
    }
}

impl From<FCLayer> for Layer {
    fn from(l: FCLayer) -> Self {
        Layer::FullyConnected(l)
    }
}

impl From<DropoutLayer> for Layer {
    fn from(l: DropoutLayer) -> Self {
        Layer::Dropout(l)
    }
}

impl From<ActivationLayer> for Layer {
    fn from(l: ActivationLayer) -> Self {
        Layer::Activation(l)
    }
}

impl From<SoftMaxLayer> for Layer {
    fn from(l: SoftMaxLayer) -> Self {
        Layer::SoftMax(l)
    }
}

impl Layer {
    pub(crate) fn number_of_neurons(&self) -> usize {
        match self {
            Layer::Input(layer) => layer.number_of_neurons(),
            Layer::FullyConnected(layer) => layer.number_of_neurons(),
            Layer::Dropout(layer) => layer.number_of_neurons(),
            Layer::Activation(layer) => layer.number_of_neurons(),
            Layer::SoftMax(layer) => layer.number_of_neurons(),
            Layer::BatchNormalize(layer) => layer.number_of_neurons(),
        }
    }

    pub fn print_summary(&self) {
        match self {
            Layer::FullyConnected(layer) => {
                layer.print_summary();
            }
            Layer::Dropout(layer) => {
                layer.print_summary();
            }
            Layer::Activation(layer) => {
                layer.print_summary();
            }
            Layer::SoftMax(layer) => {
                layer.print_summary();
            }
            Layer::BatchNormalize(layer) => {
                layer.print_summary();
            }
            _ => {}
        }
    }

    pub(crate) fn new_feedforward(&mut self) {
        if let Layer::Dropout(layer) = self {
            layer.new_feedforward();
        }
    }

    pub(crate) fn new_minibatch(&mut self, mbs: &[Minibatch], layer_index: usize) {
        if let Layer::BatchNormalize(layer) = self {
            layer.next_minibatch(mbs, layer_index);
        }
    }

    pub fn initialize(&mut self, prev_layer: &Layer) {
        match self {
            Layer::FullyConnected(layer) => {
                layer.initialize(&prev_layer);
            }
            Layer::Dropout(layer) => {
                layer.initialize(&prev_layer);
            }
            Layer::BatchNormalize(layer) => {
                layer.initialize(&prev_layer);
            }
            _ => {}
        }
    }

    pub fn feedforward(&self, layer_index: usize, mb: &mut Minibatch) {
        let input = &mb.output[layer_index - 1];
        match self {
            Layer::FullyConnected(layer) => {
                let z = layer.feedforward(&input);
                mb.output[layer_index] = z;
            }
            Layer::Dropout(layer) => {
                // SS: dropout  layer only modifies a, not z
                let a = layer.feedforward(&input);
                mb.output[layer_index] = a;
            }
            Layer::Activation(layer) => {
                let a = layer.feedforward(&input);
                mb.output[layer_index] = a;
            }
            Layer::SoftMax(layer) => {
                let a = layer.feedforward(&input);
                mb.output[layer_index] = a;
            }
            Layer::BatchNormalize(layer) => {
                let a = layer.feedforward(&input);
                mb.output[layer_index] = a;
            }
            _ => {}
        }
    }

    pub fn backprop(&self, layer_index: usize, mb: &mut Minibatch) {
        assert!(layer_index > 0);
        match self {
            FullyConnected(layer) => layer.backprop(layer_index, mb),
            Layer::Dropout(layer) => layer.backprop(layer_index, mb),
            Layer::Activation(layer) => layer.backprop(layer_index, mb),
            Layer::SoftMax(layer) => layer.backprop(layer_index, mb),
            _ => panic!(),
        }
    }

    pub fn update_network(&mut self, layer_index: usize, mbs: &[Minibatch], eta: f64, rho: f64, lambda: f64) {
        if let Layer::FullyConnected(layer) = self {
            layer.update_network(layer_index, mbs, eta, rho, lambda);
        }
    }

    pub(crate) fn weights_squared_sum(&self) -> f64 {
        match self {
            Layer::FullyConnected(layer) => layer.weights_squared_sum(),
            _ => 0.0,
        }
    }
}

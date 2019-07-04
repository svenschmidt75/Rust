#![allow(dead_code)]

use crate::ann::activation::Activation;
use crate::ann::cost_function::CostFunction;
use crate::ann::layers::dropout_layer::DropoutLayer;
use crate::ann::layers::fc_layer::FCLayer;
use crate::ann::layers::input_layer::InputLayer;
use crate::ann::layers::layer::Layer::FullyConnected;
use crate::ann::minibatch::Minibatch;
use linear_algebra::matrix::Matrix2D;
use linear_algebra::vector::Vector;

pub enum Layer {
    Input(InputLayer),
    FullyConnected(FCLayer),
    Dropout(DropoutLayer),
}

impl Layer {
    pub(crate) fn nactivations(&self) -> usize {
        match self {
            Layer::Input(layer) => layer.nactivations,
            Layer::FullyConnected(layer) => layer.nneurons,
            Layer::Dropout(layer) => layer.probability_vector.dim(),
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
            _ => {}
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
            _ => {}
        }
    }

    pub fn feedforward(&self, layer_index: usize, mb: &mut Minibatch) {
        let prev_a = &mb.a[layer_index - 1];
        match self {
            Layer::FullyConnected(layer) => {
                let (z, a) = layer.feedforward(&prev_a);
                mb.z[layer_index] = z;
                mb.a[layer_index] = a;
            }
            Layer::Dropout(layer) => {
                let (z, a) = layer.feedforward(&prev_a);
                mb.z[layer_index] = z;
                mb.a[layer_index] = a;
            }
            _ => {}
        }
    }

    pub fn calculate_outputlayer_error(&self, output_layer_index: usize, mb: &mut Minibatch, cost_function: &CostFunction, y: &Vector) {
        match self {
            FullyConnected(layer) => {
                let delta_L = layer.calculate_outputlayer_error(&mb.a[output_layer_index], &mb.z[output_layer_index], cost_function, y);
                mb.error[output_layer_index] = delta_L;
            }
            _ => panic!("Output layer error only valid for fully-connected layers"),
        }
    }

    pub fn backprop(&self, layer_index: usize, output_layer_index: usize, next_layer: &Layer, mb: &mut Minibatch) {
        assert!(layer_index > 0 && layer_index < output_layer_index);
        match self {
            FullyConnected(layer) => layer.backprop(layer_index, output_layer_index, next_layer, mb),
            Layer::Dropout(layer) => layer.backprop(layer_index, output_layer_index, next_layer, mb),
            _ => panic!(),
        }
    }

    pub(crate) fn backprop_component(&self, layer_index: usize, mb: &mut Minibatch) -> Vector {
        match self {
            FullyConnected(layer) => layer.backprop_component(layer_index, mb),
            Layer::Dropout(layer) => layer.backprop_component(layer_index, mb),
            _ => panic!(),
        }
    }
}

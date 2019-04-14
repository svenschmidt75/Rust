use crate::ann::activation::Activation;
use crate::ann::layers::training_data::TrainingData;
use crate::ann::minibatch::Minibatch;
use crate::ann::model::Model;
use crate::la::matrix::Matrix2D;
use crate::la::ops;
use crate::la::vector::Vector;

pub trait CostFunction {
    fn cost(&self, model: &mut Model, y: &Vec<TrainingData>) -> f64;

    fn output_error(&self, output_layer_index: usize, mb: &Minibatch, y: &Vector, f: &Activation) -> Vector;
}

pub struct QuadraticCost;

impl QuadraticCost {
    fn cost(a: &Vector, y: &Vector) -> f64 {
        let diff = y - a;
        let diff2 = ops::hadamard(&diff, &diff);
        diff2.iter().sum()
    }

    fn calculate_delta(model: &Model, layer_index: usize, mb: &Minibatch, x: &TrainingData) -> Vector {
        let output_layer_index = model.output_layer_index();
        if layer_index == output_layer_index {
            let layer = model.get_layer(layer_index);
            let sigma_prime = layer.get_activation().df(&mb.z[layer_index]);
            return (&mb.a[layer_index] - &x.output_activations).hadamard(&sigma_prime);
        }
        let delta_next = QuadraticCost::calculate_delta(&model, layer_index + 1, &mb, &x);
        let w_tr = model.get_weights(layer_index).transpose();
        let layer = model.get_layer(layer_index);
        let sigma_prime = layer.get_activation().df(&mb.z[layer_index]);
        w_tr.ax(&delta_next).hadamard(&sigma_prime)
    }

    fn grad_bias(model: &mut Model, layer_index: usize, xs: &[TrainingData]) -> Vector {
        assert!(layer_index > 0);
        let layer = model.get_layer(layer_index);
        let mut db = Vector::new(layer.nactivations());
        let mut mb = model.create_minibatch();

        for training_sample in xs {
            let known_classification = &training_sample.output_activations;
            mb.a[0] = training_sample.input_activations.clone();
            model.feedforward(&mut mb);
            let delta = QuadraticCost::calculate_delta(&model, layer_index, &mb, &training_sample);
            db += &delta;
        }
        db /= xs.len();
        db
    }

    fn grad_weight(model: &mut Model, layer_index: usize, xs: &[TrainingData]) -> Matrix2D {
        assert!(layer_index > 0);
        let prev_layer = model.get_layer(layer_index - 1);
        let layer = model.get_layer(layer_index);
        let mut dw = Matrix2D::new(layer.nactivations(), prev_layer.nactivations());
        let mut mb = model.create_minibatch();

        for training_sample in xs {
            let known_classification = &training_sample.output_activations;
            mb.a[0] = training_sample.input_activations.clone();
            model.feedforward(&mut mb);
            let delta = QuadraticCost::calculate_delta(&model, layer_index, &mb, &training_sample);
            let tmp = ops::outer_product(&delta, &mb.a[layer_index - 1]);
            dw += &tmp;
        }
        dw /= xs.len();
        dw
    }
}

impl CostFunction for QuadraticCost {
    fn cost(&self, model: &mut Model, y: &Vec<TrainingData>) -> f64 {
        let mut total_cost = 0.0;

        // SS: can use map and sum here...
        for x in y {
            let mut mb = model.create_minibatch();
            mb.a[0] = x.input_activations.clone();
            model.feedforward(&mut mb);
            let c = Self::cost(mb.output_activations(), &x.output_activations);
            total_cost += c;
        }
        total_cost / 2.0 / y.len() as f64
    }

    fn output_error(&self, output_layer_index: usize, mb: &Minibatch, y: &Vector, f: &Activation) -> Vector {
        let a = &mb.a[output_layer_index];
        let z = &mb.z[output_layer_index];
        assert_eq!(a.dim(), z.dim(), "Vectors must have same dimension");
        assert_eq!(a.dim(), y.dim(), "Vectors must have same dimension");

        // delta_L = grad_a C x sigma_prime of z_L, x = Hadamard
        // Formula BP1a, http://neuralnetworksanddeeplearning.com/chap2.html
        // grad_a C = a_L - y
        // sigma_prime of z_L = f.df(z)
        (a - y).hadamard(&f.df(z))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ann::layers::layer::{FCLayer, InputLayer};

    use assert_approx_eq::assert_approx_eq;

    use crate::ann::activation::Id;
    use crate::ann::activation::Sigmoid;
    use crate::ann::layers::training_data::TrainingData;
    use crate::la::matrix::Matrix2D;
    use crate::la::vector::Vector;

    #[test]
    fn test_network_cost() {
        // Arrange
        let mut model = Model::new();

        let input_layer = InputLayer::new(2);
        model.add(Box::new(input_layer));

        let hidden_layer = FCLayer::new(3, Box::new(Id {}));
        model.add(Box::new(hidden_layer));

        let output_layer = FCLayer::new(1, Box::new(Id {}));
        model.add(Box::new(output_layer));

        // Act
        let cost = QuadraticCost {};
        let training_data = vec![
            TrainingData {
                input_activations: vec![0.0, 0.0].into(),
                output_activations: vec![0.0].into(),
            },
            TrainingData {
                input_activations: vec![1.0, 0.0].into(),
                output_activations: vec![0.0].into(),
            },
            TrainingData {
                input_activations: vec![0.0, 1.0].into(),
                output_activations: vec![0.0].into(),
            },
            TrainingData {
                input_activations: vec![1.0, 1.0].into(),
                output_activations: vec![1.0].into(),
            },
        ];
        let c = cost.cost(&mut model, &training_data);

        // Assert
        assert_eq!(0.087771, c)
    }

    #[test]
    fn test_cost() {
        // Arrange
        let a = vec![1.0, 2.0].into();
        let y = vec![3.0, 4.0].into();

        // Act
        let c = QuadraticCost::cost(&a, &y);

        // Assert
        assert_eq!(8.0, c)
    }

    #[test]
    fn test_quadratic_cost_output_layer() {
        // Arrange
        let cost = QuadraticCost;
        let activation = &Sigmoid {};
        let mut mb = Minibatch::new(vec![2, 1]);

        let z1 = Vector::from(vec![1.0, 2.0]);
        mb.a[0] = activation.f(&z1);
        mb.z[0] = z1;

        let z2 = Vector::from(vec![3.0]);
        mb.a[1] = activation.f(&z2);
        mb.z[1] = z2.clone();

        let x = TrainingData {
            input_activations: Vector::from(vec![1.0, 2.0]),
            output_activations: Vector::from(vec![1.0]),
        };

        // Act
        let error = cost.output_error(1, &mb, &x.output_activations, &Sigmoid {});

        // Assert
        let d: Vector = &mb.a[1] - &x.output_activations;
        let df = &Sigmoid {}.df(&z2);
        assert_approx_eq!(d[0] * df[0], error[0], 1e-3f64);
    }
}

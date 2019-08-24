#![allow(unused_imports)]
#![allow(non_snake_case)]

use crate::ann::activation::Activation;
use crate::ann::minibatch::Minibatch;
use crate::ann::model::Model;
use crate::ann::training_data::TrainingData;
use linear_algebra::matrix::Matrix2D;
use linear_algebra::ops;
use linear_algebra::vector::Vector;

pub trait CostFunction {
    fn cost(&self, model: &mut Model, y: &[TrainingData], lambda: f64) -> f64;

    fn output_error(&self, a: &Vector, y: &Vector) -> Vector;
}

pub struct QuadraticCost;

impl QuadraticCost {
    fn single_cost(a: &Vector, y: &Vector) -> f64 {
        // SS: a are the output layer activations
        let diff = y - a;
        let diff2 = ops::hadamard(&diff, &diff);
        diff2.iter().sum::<f64>() / 2.0
    }

    fn numerical_derivative(a: &Vector, index: usize, y: &Vector) -> f64 {
        // SS: numerically calculate dC/da_{index}^{L}
        let delta = 0.000_001;

        let mut a_mut = a.clone();
        a_mut[index] = a[index] - delta;
        let c1 = QuadraticCost::single_cost(&a_mut, y);

        a_mut[index] = a[index] + delta;
        let c2 = QuadraticCost::single_cost(&a_mut, y);

        let dc = (c2 - c1) / delta / 2_f64;
        dc
    }
}

impl CostFunction for QuadraticCost {
    fn cost(&self, model: &mut Model, y: &[TrainingData], lambda: f64) -> f64 {
        let mut total_cost = 0.0;

        // SS: can use map and sum here...
        let mut mb = model.create_minibatch();
        for x in y {
            mb.output[0] = x.input_activations.clone();
            model.feedforward(&mut mb);
            let c = Self::single_cost(mb.output_activations(), &x.output_activations);
            total_cost += c;
        }
        let ntraining_samples = y.len() as f64;

        // SS: add effects of L2 regularization
        let w2 = model.weights_squared_sum();
        total_cost = (total_cost + w2 * lambda) / ntraining_samples;
        total_cost
    }

    fn output_error(&self, a: &Vector, y: &Vector) -> Vector {
        assert_eq!(a.dim(), y.dim(), "Vectors must have same dimension");

        // Formula BP1a, http://neuralnetworksanddeeplearning.com/chap2.html
        a - y
    }
}

pub struct CrossEntropyCost;

// SS: This implements categorical cross-entropy.

impl CrossEntropyCost {
    fn single_cost_i(aj: f64, yj: f64) -> f64 {
        // SS: a are the output layer activations
        let tmp1 = yj * aj.ln();
        let tmp2 = (1.0 - yj) * (1.0 - aj).ln();
        let tmp3 = tmp1 + tmp2;
        let cost = -tmp3;
        cost
    }

    fn single_cost(a: &Vector, y: &Vector) -> f64 {
        // SS: a are the output layer activations
        assert_eq!(a.dim(), y.dim(), "Vectors must have same dimension");
        let cost = (0..a.dim()).into_iter().map(|idx| CrossEntropyCost::single_cost_i(a[idx], y[idx])).sum();
        cost
    }

    fn numerical_derivative(a: &Vector, index: usize, y: &Vector) -> f64 {
        // SS: numerically calculate dC/da_{index}^{L}
        let delta = 0.000_001;

        let mut a_mut = a.clone();
        a_mut[index] = a[index] - delta;
        let c1 = CrossEntropyCost::single_cost(&a_mut, y);

        a_mut[index] = a[index] + delta;
        let c2 = CrossEntropyCost::single_cost(&a_mut, y);

        let dc = (c2 - c1) / delta / 2_f64;
        dc
    }
}

impl CostFunction for CrossEntropyCost {
    fn cost(&self, model: &mut Model, y: &[TrainingData], lambda: f64) -> f64 {
        let mut total_cost = 0.0;

        // SS: can use map and sum here...
        let mut mb = model.create_minibatch();
        for x in y {
            mb.output[0] = x.input_activations.clone();
            model.feedforward(&mut mb);
            let c = Self::single_cost(mb.output_activations(), &x.output_activations);
            total_cost += c;
        }
        let ntraining_samples = y.len() as f64;

        // SS: add effects of L2 regularization
        let w2 = model.weights_squared_sum();
        total_cost = (total_cost + w2 * lambda / 2.0) / ntraining_samples;
        total_cost
    }

    fn output_error(&self, a: &Vector, y: &Vector) -> Vector {
        assert_eq!(a.dim(), y.dim(), "Vectors must have same dimension");

        // SS: dC/d_a^{L}
        let mut dCda = Vector::new(a.dim());
        for i in 0..a.dim() {
            let ai = a[i];
            let yi = y[i];
            let t1 = (1.0 - yi) / (1.0 - ai);
            let t2 = -yi / ai;
            let t3 = t1 + t2;
            dCda[i] = t3;
        }
        dCda
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ann::layers::{fc_layer::FCLayer, input_layer::InputLayer};

    use assert_approx_eq::assert_approx_eq;

    use crate::ann::activation::Id;
    use crate::ann::activation::Sigmoid;
    use crate::ann::layers::activation_layer::ActivationLayer;
    use crate::ann::layers::layer::Layer;
    use crate::ann::training_data::TrainingData;
    use linear_algebra::matrix::Matrix2D;
    use linear_algebra::vector::Vector;

    #[test]
    fn test_network_cost() {
        // Arrange
        let mut model = Model::new();

        model.addInputLayer(InputLayer::new(2));
        model.addFullyConnectedLayer(FCLayer::new(2));
        model.addActivationLayer(ActivationLayer::new(2, Box::new(Sigmoid {})));
        model.addFullyConnectedLayer(FCLayer::new(1));
        model.addActivationLayer(ActivationLayer::new(1, Box::new(Sigmoid {})));

        let mut mb = model.create_minibatch();
        let z = Vector::from(vec![0.0, 1.0]);
        mb.output[0] = Sigmoid {}.f(&z);

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
        model.train(&data, 1000, 15.5, 0.0, 0.0, 4, &QuadraticCost {});

        // Act
        let cost = QuadraticCost {};
        let c = cost.cost(&mut model, &training_data, 0.0);

        // Assert
        assert_approx_eq!(0.00008300650113936091, c, 1E-4);
    }

    #[test]
    fn test_quadratic_cost() {
        // Arrange
        let a = vec![1.0, 2.0].into();
        let y = vec![3.0, 4.0].into();

        // Act
        let c = QuadraticCost::single_cost(&a, &y);

        // Assert
        assert_eq!(8.0, c)
    }

    #[test]
    fn test_quadratic_cost_derivative() {
        // Arrange
        let a = vec![1.5, 2.7465].into();
        let y = vec![3.3664, 4.4352].into();

        // Act
        let dc_numeric1 = QuadraticCost::numerical_derivative(&a, 0, &y);
        let dc_numeric2 = QuadraticCost::numerical_derivative(&a, 1, &y);

        // Assert
        let dc_analytical = QuadraticCost {}.output_error(&a, &y);
        assert_approx_eq!(dc_analytical[0], dc_numeric1, 1E-4);
        assert_approx_eq!(dc_analytical[1], dc_numeric2, 1E-4);
    }

    #[test]
    fn test_quadratic_cost_output_layer() {
        // Arrange
        let cost = QuadraticCost;
        let activation = &Sigmoid {};
        let mut mb = Minibatch::new(vec![2, 1]);

        let z1 = Vector::from(vec![1.0, 2.0]);
        mb.output[0] = activation.f(&z1);

        let z2 = Vector::from(vec![3.0]);
        mb.output[1] = activation.f(&z2);

        let x = TrainingData {
            input_activations: Vector::from(vec![1.0, 2.0]),
            output_activations: Vector::from(vec![1.0]),
        };

        // Act
        let error = cost.output_error(&mb.output[1], &x.output_activations);

        // Assert
        let d: Vector = &mb.output[1] - &x.output_activations;
        assert_approx_eq!(d[0], error[0], 1e-3f64);
    }

    #[test]
    fn test_cross_entropy_cost_derivative() {
        // Arrange
        let a = vec![0.512, 0.7465].into();
        let y = vec![0.3664, 0.4352].into();

        // Act
        let dc_numeric1 = CrossEntropyCost::numerical_derivative(&a, 0, &y);
        let dc_numeric2 = CrossEntropyCost::numerical_derivative(&a, 1, &y);

        // Assert
        let dc_analytical = CrossEntropyCost {}.output_error(&a, &y);
        assert_approx_eq!(dc_analytical[0], dc_numeric1, 1E-4);
        assert_approx_eq!(dc_analytical[1], dc_numeric2, 1E-4);
    }

}

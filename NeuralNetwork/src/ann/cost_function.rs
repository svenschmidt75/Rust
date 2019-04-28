use crate::ann::activation::Activation;
use crate::ann::layers::training_data::TrainingData;
use crate::ann::minibatch::Minibatch;
use crate::ann::model::Model;
use crate::la::matrix::Matrix2D;
use crate::la::ops;
use crate::la::vector::Vector;

pub trait CostFunction {
    fn cost(&self, model: &mut Model, y: &[TrainingData], lambda: f64) -> f64;

    fn output_error(&self, a: &Vector, z: &Vector, y: &Vector, f: &Activation) -> Vector;
}

pub struct QuadraticCost;

impl QuadraticCost {
    fn single_cost(a: &Vector, y: &Vector) -> f64 {
        // SS: a are the output layer activations
        let diff = y - a;
        let diff2 = ops::hadamard(&diff, &diff);
        diff2.iter().sum()
    }
}

impl CostFunction for QuadraticCost {
    fn cost(&self, model: &mut Model, y: &[TrainingData], lambda: f64) -> f64 {
        let mut total_cost = 0.0;

        // SS: can use map and sum here...
        let mut mb = model.create_minibatch();
        for x in y {
            mb.a[0] = x.input_activations.clone();
            model.feedforward(&mut mb);
            let c = Self::single_cost(mb.output_activations(), &x.output_activations);
            total_cost += c;
        }
        let ntraining_samples = y.len() as f64;

        // SS: add effects of L2 regularization
        let w2 = model.weightsSquaredSum();
        total_cost = (total_cost + w2 * lambda) / 2.0 / ntraining_samples;
        total_cost
    }

    fn output_error(&self, a: &Vector, z: &Vector, y: &Vector, f: &Activation) -> Vector {
        assert_eq!(a.dim(), z.dim(), "Vectors must have same dimension");
        assert_eq!(a.dim(), y.dim(), "Vectors must have same dimension");

        // delta_L = grad_a C x sigma_prime of z_L, x = Hadamard
        // Formula BP1a, http://neuralnetworksanddeeplearning.com/chap2.html
        // grad_a C = a_L - y
        // sigma_prime of z_L = f.df(z)
        (a - y).hadamard(&f.df(z))
    }
}

pub struct CrossEntropyCost;

// SS: This implements categorical cross-entropy.

impl CrossEntropyCost {
    fn single_cost(a: &Vector, y: &Vector) -> f64 {
        // SS: a are the output layer activations
        assert_eq!(a.dim(), y.dim(), "Vectors must have same dimension");
        let mut cost = 0.0;
        for idx in 0..a.dim() {
            let a_j = a[idx];
            let y_j = y[idx];
            let tmp1 = y_j * a_j.log10();
            let tmp2 = (1.0 - y_j) * (1.0 - a_j).log10();
            let tmp3 = tmp1 + tmp2;
            cost += tmp3;
        }
        cost
    }
}

impl CostFunction for CrossEntropyCost {
    fn cost(&self, model: &mut Model, y: &[TrainingData], lambda: f64) -> f64 {
        let mut total_cost = 0.0;

        // SS: can use map and sum here...
        let mut mb = model.create_minibatch();
        for x in y {
            mb.a[0] = x.input_activations.clone();
            model.feedforward(&mut mb);
            let c = Self::single_cost(mb.output_activations(), &x.output_activations);
            total_cost += c;
        }
        let ntraining_samples = y.len() as f64;

        // SS: add effects of L2 regularization
        let w2 = model.weightsSquaredSum();
        total_cost = (total_cost + w2 * lambda / 2.0) / ntraining_samples;
        -total_cost
    }

    fn output_error(&self, a: &Vector, _z: &Vector, y: &Vector, _f: &Activation) -> Vector {
        // Note: This makes only sense when the sigmoid function is used in the output later!
        // Otherwise, we still need to multiply by its derivative
        a - y
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

        let hidden_layer = FCLayer::new(2, Box::new(Sigmoid {}));
        model.add(Box::new(hidden_layer));

        let output_layer = FCLayer::new(1, Box::new(Sigmoid {}));
        model.add(Box::new(output_layer));

        let mut mb = model.create_minibatch();
        mb.z[0] = Vector::from(vec![0.0, 1.0]);
        mb.a[0] = Sigmoid {}.f(&mb.z[0]);

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
        model.train(&data, 1000, 15.5, 0.0, 4, &QuadraticCost {});

        // Act
        let cost = QuadraticCost {};
        let c = cost.cost(&mut model, &training_data, 0.0);

        // Assert
        assert_approx_eq!(0.00008300650113936091, c, 1E-4);
    }

    #[test]
    fn test_cost() {
        // Arrange
        let a = vec![1.0, 2.0].into();
        let y = vec![3.0, 4.0].into();

        // Act
        let c = QuadraticCost::single_cost(&a, &y);

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
        let error = cost.output_error(&mb.a[1], &mb.z[1], &x.output_activations, &Sigmoid {});

        // Assert
        let d: Vector = &mb.a[1] - &x.output_activations;
        let df = &Sigmoid {}.df(&z2);
        assert_approx_eq!(d[0] * df[0], error[0], 1e-3f64);
    }
}

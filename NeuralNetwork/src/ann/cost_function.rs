use crate::ann::activation::Activation;
use crate::ann::layers::training_data::TrainingData;
use crate::ann::minibatch::Minibatch;
use crate::ann::model::Model;
use crate::la::matrix::Matrix2D;
use crate::la::ops;
use crate::la::vector::Vector;

pub trait CostFunction {
    fn cost(&self, model: &mut Model, y: &[TrainingData]) -> f64;

    fn output_error(&self, a: &Vector, y: &Vector) -> Vector;
}

pub struct QuadraticCost;

impl QuadraticCost {
    fn cost(a: &Vector, y: &Vector) -> f64 {
        let diff = y - a;
        let diff2 = ops::hadamard(&diff, &diff);
        diff2.iter().sum()
    }
}

impl CostFunction for QuadraticCost {
    fn cost(&self, model: &mut Model, y: &[TrainingData]) -> f64 {
        let mut total_cost = 0.0;

        // SS: can use map and sum here...
        let mut mb = model.create_minibatch();
        for x in y {
            mb.a[0] = x.input_activations.clone();
            model.feedforward(&mut mb);
            let c = Self::cost(mb.output_activations(), &x.output_activations);
            total_cost += c;
        }
        total_cost / 2.0 / y.len() as f64
    }

    fn output_error(&self, a: &Vector, y: &Vector) -> Vector {
        // delta_L = grad_a C x sigma_prime of z_L, x = Hadamard
        // Formula BP1a, http://neuralnetworksanddeeplearning.com/chap2.html
        // grad_a C = a_L - y
        // sigma_prime of z_L = f.df(z)
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
        model.train(&data, 1000, 15.5, 1.0, 4, &QuadraticCost {});

        // Act
        let cost = QuadraticCost {};
        let c = cost.cost(&mut model, &training_data);

        // Assert
        assert_approx_eq!(0.00008300650113936091, c, 1E-4);
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
        let d_cost = cost.output_error(&mb.a[1], &x.output_activations);
        let error = d_cost.hadamard(&Sigmoid {}.df(&mb.z[1]));

        // Assert
        let d: Vector = &mb.a[1] - &x.output_activations;
        let df = &Sigmoid {}.df(&z2);
        assert_approx_eq!(d[0] * df[0], error[0], 1e-3f64);
    }
}

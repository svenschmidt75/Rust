use crate::ann::layers::training_data::TrainingData;
use crate::ann::minibatch::Minibatch;
use crate::ann::model::Model;
use crate::la::ops;
use crate::la::vector::Vector;

pub trait CostFunction {
    fn cost(&self, model: &mut Model, y: &Vec<TrainingData>) -> f64;
}

struct QuadraticCost {}

fn cost(a: &Vector, y: &Vector) -> f64 {
    let diff = y - a;
    let diff2 = ops::hadamard(&diff, &diff);
    diff2.iter().sum()
}

impl CostFunction for QuadraticCost {
    fn cost(&self, model: &mut Model, y: &Vec<TrainingData>) -> f64 {
        let mut total_cost = 0.0;

        // SS: can use map and sum here...
        for x in y {
            let mut mb = Minibatch::new();
            mb.set_input_activation(x.input_activations.clone());
            model.feedforward(&mut mb);
            let c = cost(&x.output_activations, mb.output_activation());
            total_cost += c;
        }
        total_cost / 2.0 / y.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::ann::activation;
    use crate::ann::activation::Id;
    use crate::ann::activation::ReLU;
    use crate::ann::activation::Sigmoid;
    use crate::ann::layers::fc_layer::FCLayer;
    use crate::ann::layers::training_data::TrainingData;
    use crate::la::matrix::Matrix;
    use crate::la::vector::Vector;

    use super::*;

    #[test]
    fn test_network_cost() {
        // Arrange
        let mut model = Model::new();
        let weights1 = Matrix::new_from_data(3, 2, vec![0.01, 0.02, 0.03, 0.04, 0.05, 0.06]);
        let biases1: Vector = vec![0.1, 0.2, 0.3].into();
        let hidden_layer = FCLayer::new(weights1.clone(), biases1.clone(), Box::new(Id {}));
        model.add(Box::new(hidden_layer));

        let weights2 = Matrix::new_from_data(1, 3, vec![0.1, 0.2, 0.3]);
        let biases2: Vector = vec![0.1].into();
        let output_layer = FCLayer::new(weights2.clone(), biases2.clone(), Box::new(Id {}));
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
        let c = cost(&a, &y);

        // Assert
        assert_eq!(8.0, c)
    }
}

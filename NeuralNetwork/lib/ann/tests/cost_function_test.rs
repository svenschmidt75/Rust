use ann::ann::activation::Sigmoid;
use ann::ann::cost_function::{CostFunction, QuadraticCost};
use ann::ann::layers::activation_layer::ActivationLayer;
use ann::ann::layers::fc_layer::FCLayer;
use ann::ann::layers::input_layer::InputLayer;
use ann::ann::model::Model;
use ann::ann::training_data::TrainingData;
use assert_approx_eq::assert_approx_eq;
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
    mb.output[0] = Vector::from(vec![0.5, 0.7310585786300049]);

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

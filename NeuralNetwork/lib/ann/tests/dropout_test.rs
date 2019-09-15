use ann::ann::activation::Sigmoid;
use ann::ann::cost_function::{CrossEntropyCost, QuadraticCost};
use ann::ann::layers::activation_layer::ActivationLayer;
use ann::ann::layers::dropout_layer::{DropoutLayer, ProdUniformDistributionSampler};
use ann::ann::layers::fc_layer::FCLayer;
use ann::ann::layers::input_layer::InputLayer;
use ann::ann::model::Model;
use ann::ann::training_data::TrainingData;
use mnist_loader::loader::{load_image_file, load_label_file};

const PROJECT_DIRECTORY: &str = "/home/svenschmidt75/Develop/Rust/NeuralNetwork/lib/ann/src/ann/";

#[test]
fn test_dropout_mnist() {
    // Arrange
    let training_images = load_image_file(&(PROJECT_DIRECTORY.to_owned() + "../../../../MNIST/train-images.idx3-ubyte"))
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    let training_labels = load_label_file(&(PROJECT_DIRECTORY.to_owned() + "../../../../MNIST/train-labels.idx1-ubyte"))
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();

    let test_images = load_image_file(&(PROJECT_DIRECTORY.to_owned() + "../../../../MNIST/t10k-images.idx3-ubyte"))
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();
    let test_labels = load_label_file(&(PROJECT_DIRECTORY.to_owned() + "../../../../MNIST/t10k-labels.idx1-ubyte"))
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();

    let training_data = training_images
        .iter()
        .zip(training_labels.iter())
        .map(|data| TrainingData::from_mnist(&data.0.data, data.1.label))
        //        .take(1000)
        .collect::<Vec<_>>();

    let test_data = test_images
        .iter()
        .zip(test_labels.iter())
        .map(|data| TrainingData::from_mnist(&data.0.data, data.1.label))
        .collect::<Vec<_>>();
    //        let partitioned_data = TrainingData::partition(&training_data, 0.8, 0.2);
    let partitioned_data = (&training_data[..], &training_data[0..0], &test_data[..]);

    // SS: set up model
    let mut model = Model::new();
    model.addInputLayer(InputLayer::new(28 * 28));
    model.addFullyConnectedLayer(FCLayer::new(100));
    model.addActivationLayer(ActivationLayer::new(100, Box::new(Sigmoid {})));
    model.addDropoutLayer(DropoutLayer::new(100, 0.5, Box::new(ProdUniformDistributionSampler::new())));
    model.addFullyConnectedLayer(FCLayer::new(10));
    model.addActivationLayer(ActivationLayer::new(10, Box::new(Sigmoid {})));

    let cost_function = QuadraticCost;

    // Act
    model.train(&partitioned_data, 50, 2.5, 0.0, 0.00001, 25, &cost_function);

    // Assert
}

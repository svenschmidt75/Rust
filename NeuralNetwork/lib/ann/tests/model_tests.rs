use rand::Rng;

use ann::ann::activation::{Id, Sigmoid, Sin};
use ann::ann::cost_function::QuadraticCost;
use ann::ann::layers::activation_layer::ActivationLayer;
use ann::ann::layers::fc_layer::FCLayer;
use ann::ann::layers::input_layer::InputLayer;
use ann::ann::layers::softmax_layer::SoftMaxLayer;
use ann::ann::model::Model;
use ann::ann::training_data::TrainingData;
use assert_approx_eq::assert_approx_eq;
use linear_algebra::vector::Vector;
use mnist_loader::loader::{load_image_file, load_label_file};

const PROJECT_DIRECTORY: &'static str = "/home/svenschmidt75/Develop/Rust/NeuralNetwork/lib/ann/src/ann/";

#[test]
fn test_train_1() {
    /* Train f(x) = u1 * sin(x) + u2 * sin(x) + b2 where x is the input activation and
     * sin is the activation function of the hidden layer. Id is the activation function
     * for the output layer.
     * Basically, z^{2}_{0} = sigma2(w2_0 * sigma1(x) + w2_1 * sigma1(x)) + b2, where w=1 and b=0.
     */

    // Arrange
    let mut model = Model::new();

    model.addInputLayer(InputLayer::new(1));
    model.addFullyConnectedLayer(FCLayer::new(2));
    model.addActivationLayer(ActivationLayer::new(2, Box::new(Sin {})));
    model.addFullyConnectedLayer(FCLayer::new(1));
    model.addActivationLayer(ActivationLayer::new(1, Box::new(Id {})));

    // SS: restrict input to (-pi/2, pi/2) because of periodicity
    let w2_0 = 1.2;
    let w2_1 = 0.87;
    let b2 = -1.1;
    let ntraining_samples = 1000;
    let step = std::f64::consts::PI / ntraining_samples as f64;
    let training_data = (0..ntraining_samples)
        .map(|x| ((x as f64 - ntraining_samples as f64 / 2.0) * step))
        .map(|x| TrainingData {
            input_activations: Vector::from(vec![x]),
            output_activations: Vector::from(vec![w2_0 * x.sin() + w2_1 * x.sin() + b2]),
        })
        .collect::<Vec<_>>();
    let tmp: [TrainingData; 0] = [];
    let data = (&training_data[..], &tmp as &[TrainingData], &tmp as &[TrainingData]);

    // Act
    model.train(&data, 100, 0.02, 0.0, 0.0, 25, &QuadraticCost {});

    // Assert
    let output_layer_index = 4;
    let mut mb = model.create_minibatch();
    let mut rng = rand::thread_rng();
    let result = (0..50_usize)
        .map(|_| rng.gen::<usize>() % ntraining_samples)
        .map(|idx| {
            let td = &training_data[idx];
            mb.output[0] = td.input_activations.clone();
            model.feedforward(&mut mb);
            (&mb.output[output_layer_index][0] - td.output_activations[0]).abs()
        })
        .fold(true, |acc, len| acc && len < 0.1);
    assert!(result);
}

#[test]
fn test_train_2() {
    use ann::ann::activation::{Id, Sin};

    /* Train f(x) = u1 * sin(x) + u2 * sin(x) + b2 where x is the input activation and
     * sin is the activation function of the hidden layer. Id is the activation function
     * for the output layer.
     * Basically, z^{2}_{0} = sigma2(w2_0 * sigma1(x) + w2_1 * sigma1(x)) + b2, where w=1 and b=0.
     */

    // Arrange
    let mut model = Model::new();

    model.addInputLayer(InputLayer::new(1));
    model.addFullyConnectedLayer(FCLayer::new(2));
    model.addActivationLayer(ActivationLayer::new(2, Box::new(Sin {})));
    model.addFullyConnectedLayer(FCLayer::new(1));
    model.addActivationLayer(ActivationLayer::new(1, Box::new(Id {})));

    // SS: restrict input to (-pi/2, pi/2) because of periodicity
    let w2_0 = 1.2;
    let w2_1 = 0.87;
    let b2 = -1.1;
    let ntraining_samples = 1000;
    let step = std::f64::consts::PI / ntraining_samples as f64;
    let training_data = (0..ntraining_samples)
        .map(|x| ((x as f64 - ntraining_samples as f64 / 2.0) * step))
        .map(|x| TrainingData {
            input_activations: Vector::from(vec![x]),
            output_activations: Vector::from(vec![w2_0 * x.sin() + w2_1 * x.sin() + b2]),
        })
        .collect::<Vec<_>>();
    let tmp: [TrainingData; 0] = [];
    let data = (&training_data[..], &tmp as &[TrainingData], &tmp as &[TrainingData]);

    // Act
    model.train(&data, 100, 0.02, 0.0, 0.0, 25, &QuadraticCost {});

    // Assert
    let output_layer_index = 4;
    let mut mb = model.create_minibatch();
    let mut rng = rand::thread_rng();
    let result = (0..50_usize)
        .map(|_x| rng.gen::<usize>() % ntraining_samples)
        .map(|idx| {
            let td = &training_data[idx];
            mb.output[0] = td.input_activations.clone();
            model.feedforward(&mut mb);
            (&mb.output[output_layer_index][0] - td.output_activations[0]).abs()
        })
        .fold(true, |acc, len| acc && len < 0.1);
    assert!(result);
}

#[test]
fn test_train_and_gate() {
    /* Train network to learn AND gate, i.e.
     *  a1 | a2 | a1 & a2
     *   0 |  0 |    0
     *   0 |  1 |    0
     *   1 |  0 |    0
     *   1 |  1 |    1
     */

    // Arrange

    let cost_function = QuadraticCost;

    let mut model = Model::new();

    model.addInputLayer(InputLayer::new(2));
    model.addFullyConnectedLayer(FCLayer::new(2));
    model.addActivationLayer(ActivationLayer::new(2, Box::new(Sigmoid {})));
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
    model.train(&data, 1000, 50.0, 0.0, 0.00001, 4, &cost_function);

    // Assert
    let mut mb = model.create_minibatch();

    // 0 && 0 == 0
    mb.output[0] = Vector::from(vec![0.0, 0.0]);
    model.feedforward(&mut mb);
    assert_approx_eq!(0.0003339, &mb.output[4][0], 0.001);

    // 1 && 0 == 0
    mb.output[0] = Vector::from(vec![1.0, 0.0]);
    model.feedforward(&mut mb);
    assert_approx_eq!(0.00656, &mb.output[4][0], 0.01);

    // 0 && 1 == 0
    mb.output[0] = Vector::from(vec![0.0, 1.0]);
    model.feedforward(&mut mb);
    assert_approx_eq!(0.005284975257848634, &mb.output[4][0], 0.01);

    // 1 && 1 == 0
    mb.output[0] = Vector::from(vec![1.0, 1.0]);
    model.feedforward(&mut mb);
    assert_approx_eq!(0.9887443090898671, &mb.output[4][0], 0.01);
}

#[test]
fn test_mnist() {
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
        .take(1000)
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
    model.addFullyConnectedLayer(FCLayer::new(10));
    model.addActivationLayer(ActivationLayer::new(10, Box::new(Sigmoid {})));

    let cost_function = QuadraticCost;

    // Act
    model.train(&partitioned_data, 50, 2.5, 0.0, 0.00001, 25, &cost_function);

    // Assert
}

#[test]
fn test_mnist_softmax() {
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
        .take(1000)
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
    model.addFullyConnectedLayer(FCLayer::new(10));
    model.addSoftMaxLayer(SoftMaxLayer::new(10));

    let cost_function = QuadraticCost;

    // Act
    model.train(&partitioned_data, 50, 2.5, 0.0, 0.00001, 25, &cost_function);

    // Assert
}

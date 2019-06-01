use assert_approx_eq::assert_approx_eq;
use linear_algebra::vector::Vector;

pub struct TrainingData {
    pub input_activations: Vector,
    pub output_activations: Vector,
}

impl TrainingData {
    pub fn from_mnist(data: &[u8], label: u8) -> TrainingData {
        TrainingData {
            input_activations: Vector::from(data.iter().map(|&x| x as f64 / 255_f64).collect::<Vec<_>>()),
            output_activations: Vector::from((0..10).map(|x| if x == label { 1__f64 } else { 0_f64 }).collect::<Vec<_>>()),
        }
    }

    pub fn partition(self, train: f64, test: f64) -> (TrainingData, TrainingData, TrainingData) {
        assert!(train > 0.0 && train <= 1.0);
        assert!(test >= 0.0 && test < 1.0);
        let validate = 1.0 - train - test;
        assert!(validate >= 0.0 && validate < 1.0);
        assert_approx_eq!(train + test + validate, 1.0, 1E-8);

        let validate_percent = validate / (1.0 - train);

        let (training_input_data, remainder_input) = self.input_activations.partition(train);
        let (validate_input_data, test_input_data) = remainder_input.partition(validate_percent);

        let (training_output_data, remainder_output) = self.output_activations.partition(train);
        let (validate_output_data, test_output_data) = remainder_output.partition(validate_percent);

        (
            TrainingData {
                input_activations: training_input_data,
                output_activations: training_output_data,
            },
            TrainingData {
                input_activations: validate_input_data,
                output_activations: validate_output_data,
            },
            TrainingData {
                input_activations: test_input_data,
                output_activations: test_output_data,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Arrange
        let data = TrainingData {
            input_activations: Vector::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]),
            output_activations: Vector::from(vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5, 10.5]),
        };

        // Act
        let (train, validate, test) = data.partition(0.6, 0.2);

        // Assert
        assert_eq!(train.input_activations.dim(), 6);
        assert_eq!(validate.input_activations.dim(), 2);
        assert_eq!(test.input_activations.dim(), 2);
    }
}

#![allow(dead_code)]

use assert_approx_eq::assert_approx_eq;
use linear_algebra::vector::Vector;

pub struct TrainingData {
    pub input_activations: Vector,
    pub output_activations: Vector,
}

impl TrainingData {
    pub fn from_mnist(data: &[u8], label: u8) -> TrainingData {
        TrainingData {
            input_activations: Vector::from(data.iter().map(|&x| f64::from(x) / 255_f64).collect::<Vec<_>>()),
            output_activations: Vector::from((0..10).map(|x| if x == label { 1_f64 } else { 0_f64 }).collect::<Vec<_>>()),
        }
    }

    pub fn partition(data: &[TrainingData], train: f64, test: f64) -> (&[TrainingData], &[TrainingData], &[TrainingData]) {
        assert!(train > 0.0 && train <= 1.0);
        assert!(test >= 0.0 && test < 1.0);
        let validate = 1.0 - train - test;
        //        assert!(validate >= 0.0 && validate < 1.0);
        assert_approx_eq!(train + test + validate, 1.0, 1E-8);

        let train_length = (data.len() as f64 * train) as usize;
        let test_length = (data.len() as f64 * test) as usize;
        let validate_length = data.len() - train_length - test_length;

        let validate_offset = train_length;
        let test_offset = train_length + validate_length;
        (&data[0..train_length], &data[validate_offset..validate_offset + validate_length], &data[test_offset..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without_validation() {
        // Arrange
        let data = (0..100)
            .map(|_| TrainingData {
                input_activations: Vector::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]),
                output_activations: Vector::from(vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5, 10.5]),
            })
            .collect::<Vec<_>>();

        // Act
        let (train, validate, test) = TrainingData::partition(&data, 0.8, 0.2);

        // Assert
        assert_eq!(train.len(), 80);
        assert_eq!(validate.len(), 0);
        assert_eq!(test.len(), 20);
    }

    #[test]
    fn test_with_validation() {
        // Arrange
        let data = (0..100)
            .map(|_| TrainingData {
                input_activations: Vector::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]),
                output_activations: Vector::from(vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5, 10.5]),
            })
            .collect::<Vec<_>>();

        // Act
        let (train, validate, test) = TrainingData::partition(&data, 0.6, 0.2);

        // Assert
        assert_eq!(train.len(), 60);
        assert_eq!(validate.len(), 20);
        assert_eq!(test.len(), 20);
    }
}

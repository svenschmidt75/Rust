use crate::la::vector::Vector;
use crate::la::ops;

pub trait Activation {
    fn f(&self, v: Vector) -> Vector;
}

pub struct Sigmoid {}

pub fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

impl Activation for Sigmoid {

    fn f(&self, v: Vector) -> Vector {
        ops::f(v, sigmoid)
    }

}

pub struct ReLU {}

pub fn relu(z: f64) -> f64 {
    z.max(0.0)
}

impl Activation for ReLU {

    fn f(&self, v: Vector) -> Vector {
        ops::f(v, relu)
    }

}

pub struct LeakyReLU {}

pub fn leaky_relu(z: f64) -> f64 {
    z.max(0.01 * z)
}

impl Activation for LeakyReLU {

    fn f(&self, v: Vector) -> Vector {
        ops::f(v, leaky_relu)
    }

}

pub struct Tanh {}

pub fn tanh(z: f64) -> f64 {
    2.0 / (1.0 + (-2.0 * z).exp()) - 1.0
}

impl Activation for Tanh {

    fn f(&self, v: Vector) -> Vector {
        ops::f(v, tanh)
    }

}

pub struct SoftMax {}

impl Activation for SoftMax {

    fn f(&self, v: Vector) -> Vector {
        let denominator: f64 = v.iter().map(|x| x.exp()).sum();
        let result: Vector = v.iter().map(|x| x.exp() / denominator).collect::<Vec<_>>().into();
        result
    }

}

pub struct Id {}

impl Activation for Id {

    fn f(&self, v: Vector) -> Vector {
        v
    }

}

// SS: test data from https://keisan.casio.com/menu/system/000000001350

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid() {
        // Arrange
        // Act
        let result = sigmoid(2.03);

        // Assert
        assert_eq!(0.8839110779310051, result)
    }

    #[test]
    fn test_relu_negative() {
        // Arrange
        // Act
        let result = relu(-2.03);

        // Assert
        assert_eq!(0.0, result)
    }

    #[test]
    fn test_relu_positive() {
        // Arrange
        // Act
        let result = relu(2.03);

        // Assert
        assert_eq!(2.03, result)
    }

    #[test]
    fn test_leaky_relu_negative() {
        // Arrange
        // Act
        let result = leaky_relu(-2.03);

        // Assert
        assert_eq!(-2.03 * 0.01, result)
    }

    #[test]
    fn test_leaky_relu_positive() {
        // Arrange
        // Act
        let result = leaky_relu(2.03);

        // Assert
        assert_eq!(2.03, result)
    }

    #[test]
    fn test_tanh() {
        // Arrange
        // Act
        let result = tanh(2.03);

        // Assert
        assert_eq!(0.9660869289795986, result)
    }

    #[test]
    fn test_softmax() {
        // Arrange
        let values = vec![3.0, 4.0, 1.0];

        // Act
        let result = SoftMax{}.f(values.into());

        // Assert
        assert_eq!(0.259496460342419118, result[0]);
        assert_eq!(0.7053845126982411, result[1]);
        assert_eq!(0.0351190269593397242, result[2])
    }

}

use crate::la::ops;
use crate::la::vector::Vector;

pub trait Activation {
    fn f(&self, v: &Vector) -> Vector;

    fn df(&self, v: &Vector) -> Vector;
}

pub struct Sigmoid {}

pub fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

impl Activation for Sigmoid {
    fn f(&self, v: &Vector) -> Vector {
        ops::f(v, &sigmoid)
    }

    fn df(&self, v: &Vector) -> Vector {
        let s = ops::f(v, &sigmoid);
//        s * (1.0 - s)
        s
    }
}

pub struct ReLU {}

pub fn relu(z: f64) -> f64 {
    z.max(0.0)
}

pub fn drelu(z: f64) -> f64 {
    match z < 0.0 {
        false => 1.0,
        _     => 0.0
    }
}

impl Activation for ReLU {
    fn f(&self, v: &Vector) -> Vector {
        ops::f(v, &relu)
    }

    // todo SS: verify this
    fn df(&self, v: &Vector) -> Vector {
        ops::f(v, &drelu)
    }
}

pub struct LeakyReLU {}

pub fn leaky_relu(z: f64) -> f64 {
    z.max(0.01 * z)
}

impl Activation for LeakyReLU {
    fn f(&self, v: &Vector) -> Vector {
        ops::f(v, &leaky_relu)
    }

    // todo SS: verify this
    fn df(&self, v: &Vector) -> Vector {
        ops::f(v, &leaky_relu)
    }
}

pub struct Tanh {}

pub fn tanh(z: f64) -> f64 {
    2.0 / (1.0 + (-2.0 * z).exp()) - 1.0
}

impl Activation for Tanh {
    fn f(&self, v: &Vector) -> Vector {
        ops::f(v, &tanh)
    }

    // todo SS: verify this
    fn df(&self, v: &Vector) -> Vector {
        ops::f(v, &tanh)
    }
}

pub struct SoftMax {}

impl Activation for SoftMax {
    fn f(&self, v: &Vector) -> Vector {
        let denominator: f64 = v.iter().map(|x| x.exp()).sum();
        let result: Vector = v
            .iter()
            .map(|x| x.exp() / denominator)
            .collect::<Vec<_>>()
            .into();
        result
    }

    // todo SS: verify this
    fn df(&self, v: &Vector) -> Vector {
        ops::f(v, &tanh)
    }
}

pub struct Id {}

impl Activation for Id {
    fn f(&self, v: &Vector) -> Vector {
        v.clone()
    }

    fn df(&self, v: &Vector) -> Vector {
        Vector::from((0..v.dim()).map(|_| 0.0).collect::<Vec<_>>())
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
        let result = SoftMax {}.f(&values.into());

        // Assert
        assert_eq!(0.259496460342419118, result[0]);
        assert_eq!(0.7053845126982411, result[1]);
        assert_eq!(0.0351190269593397242, result[2])
    }

}

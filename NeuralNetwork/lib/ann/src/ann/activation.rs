#![allow(dead_code)]

use linear_algebra::ops;
use linear_algebra::vector::Vector;

pub trait Activation {
    fn f(&self, v: &Vector) -> Vector;

    fn df(&self, v: &Vector) -> Vector;
}

pub struct Sigmoid;

pub(crate) fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

pub(crate) fn sigmoid_prime(z: f64) -> f64 {
    sigmoid(z) * (1.0 - sigmoid(z))
}

impl Activation for Sigmoid {
    fn f(&self, v: &Vector) -> Vector {
        ops::f(v, &sigmoid)
    }

    fn df(&self, v: &Vector) -> Vector {
        ops::f(v, &sigmoid_prime)
    }
}

pub struct ReLU;

pub(crate) fn relu(z: f64) -> f64 {
    z.max(0.0)
}

pub(crate) fn relu_prime(z: f64) -> f64 {
    if z < 0.0 {
        0.0
    } else {
        1.0
    }
}

impl Activation for ReLU {
    fn f(&self, v: &Vector) -> Vector {
        ops::f(v, &relu)
    }

    // todo SS: verify this
    fn df(&self, v: &Vector) -> Vector {
        ops::f(v, &relu_prime)
    }
}

pub struct LeakyReLU;

pub(crate) fn leaky_relu(z: f64) -> f64 {
    if z <= 0.0 {
        0.01 * z
    } else {
        z
    }
}

pub(crate) fn leaky_relu_prime(z: f64) -> f64 {
    if z <= 0.0 {
        0.01
    } else {
        1.0
    }
}

impl Activation for LeakyReLU {
    fn f(&self, v: &Vector) -> Vector {
        ops::f(v, &leaky_relu)
    }

    fn df(&self, v: &Vector) -> Vector {
        ops::f(v, &leaky_relu_prime)
    }
}

pub struct Tanh;

pub(crate) fn tanh(z: f64) -> f64 {
    2.0 / (1.0 + (-2.0 * z).exp()) - 1.0
}

pub(crate) fn tanh_prime(z: f64) -> f64 {
    let tanh = tanh(z);
    1.0 - tanh * tanh
}

impl Activation for Tanh {
    fn f(&self, v: &Vector) -> Vector {
        ops::f(v, &tanh)
    }

    fn df(&self, v: &Vector) -> Vector {
        ops::f(v, &tanh_prime)
    }
}

pub struct SoftMax;

impl SoftMax {

    fn derivative(&self, v: &Vector, idx: usize) -> f64 {
        let delta = 0.000_001;

        let mut v_mut = v.clone();
        v_mut[idx] = v[idx] - delta;
        let c1 = self.f(&v_mut);

        v_mut[idx] = v[idx] + delta;
        let c2 = self.f(&v_mut);

        let df = (c2[idx] - c1[idx]) / delta / 2_f64;
        df
    }

}

impl Activation for SoftMax {
    fn f(&self, v: &Vector) -> Vector {
        let denominator: f64 = v.iter().map(|x| x.exp()).sum();
        let result: Vector = v.iter().map(|x| x.exp() / denominator).collect::<Vec<_>>().into();
        result
    }

    fn df(&self, v: &Vector) -> Vector {
        let f1 = <Self as Activation>::f(self, v);
        let result: Vector = f1.iter().map(|x| 1.0 - *x).collect::<Vec<_>>().into();
        ops::hadamard(&f1, &result)
    }
}

pub struct Id;

impl Activation for Id {
    fn f(&self, v: &Vector) -> Vector {
        v.clone()
    }

    fn df(&self, v: &Vector) -> Vector {
        Vector::from(vec![1.0; v.dim()])
    }
}

pub struct Sin;

pub(crate) fn sin(z: f64) -> f64 {
    z.sin()
}

pub(crate) fn cos(z: f64) -> f64 {
    z.cos()
}

impl Activation for Sin {
    fn f(&self, v: &Vector) -> Vector {
        ops::f(v, &sin)
    }

    fn df(&self, v: &Vector) -> Vector {
        ops::f(v, &cos)
    }
}

// SS: test data from https://keisan.casio.com/menu/system/000000001350

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use linear_algebra::vector::Vector;

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
    fn test_sigmoid_prime() {
        // Arrange
        let h = 0.001;

        // Act
        let f1 = sigmoid(2.03 + h);
        let f2 = sigmoid(2.03);
        let df = (f1 - f2) / h;

        // Assert
        assert_approx_eq!(sigmoid_prime(2.03), df, h)
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
    fn test_relu_prime_negative() {
        // Arrange
        let h = 0.001;

        // Act
        let f1 = relu(-2.03 + h);
        let f2 = relu(-2.03);
        let df = (f1 - f2) / h;

        // Assert
        assert_approx_eq!(relu_prime(-2.03), df, h)
    }

    #[test]
    fn test_relu_prime_positive() {
        // Arrange
        let h = 0.001;

        // Act
        let f1 = relu(2.03 + h);
        let f2 = relu(2.03);
        let df = (f1 - f2) / h;

        // Assert
        assert_approx_eq!(relu_prime(2.03), df, h)
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
    fn test_leaky_relu_prime_negative() {
        // Arrange
        let h = 0.001;

        // Act
        let f1 = leaky_relu(-2.03 + h);
        let f2 = leaky_relu(-2.03);
        let df = (f1 - f2) / h;

        // Assert
        assert_approx_eq!(leaky_relu_prime(-2.03), df, h)
    }

    #[test]
    fn test_leaky_relu_prime_positive() {
        // Arrange
        let h = 0.001;

        // Act
        let f1 = leaky_relu(2.03 + h);
        let f2 = leaky_relu(2.03);
        let df = (f1 - f2) / h;

        // Assert
        assert_approx_eq!(leaky_relu_prime(2.03), df, h)
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
    fn test_tanh_prime() {
        // Arrange
        let h = 0.001;

        // Act
        let f1 = tanh(2.03 + h);
        let f2 = tanh(2.03);
        let df = (f1 - f2) / h;

        // Assert
        assert_approx_eq!(tanh_prime(2.03), df, h)
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

    #[test]
    fn t() {
        // Arrange
        let values: Vector = vec![2.0, 1.0, 0.1].into();

        let df1 = SoftMax{}.derivative(&values, 0);
        let df2 = SoftMax{}.derivative(&values, 1);
        let df3 = SoftMax{}.derivative(&values, 2);

        // Act
        let df_analytic = SoftMax{}.df(&values);

        // Assert
        assert_approx_eq!(df1, df_analytic[0], 1e-3f64);
        assert_approx_eq!(df2, df_analytic[1], 1e-3f64);
        assert_approx_eq!(df3, df_analytic[2], 1e-3f64);
    }
}

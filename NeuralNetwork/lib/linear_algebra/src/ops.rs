use crate::matrix::Matrix2D;
use crate::vector::Vector;

pub fn ax(m: &Matrix2D, x: &Vector) -> Vector {
    assert_eq!(m.ncols(), x.dim(), "ops.ax: Matrix incompatible with vector");
    let mut ax = Vector::new(m.nrows());
    for ri in 0..m.nrows() {
        let mut tmp = 0.0;
        for ci in 0..m.ncols() {
            tmp += m[(ri, ci)] * x[ci];
        }
        ax[ri] = tmp;
    }
    ax
}

pub fn f<F: Fn(f64) -> f64>(v: &Vector, f: &F) -> Vector {
    let result: Vector = v.iter().map(|&x| f(x)).collect::<Vec<_>>().into();
    result
}

pub fn hadamard(v1: &Vector, v2: &Vector) -> Vector {
    assert_eq!(v1.dim(), v2.dim(), "Vectors must have same dimension");
    let output: Vec<_> = v1.iter().zip(v2.iter()).map(|(&x1, &x2)| x1 * x2).collect();
    output.into()
}

pub fn outer_product(v1: &Vector, v2: &Vector) -> Matrix2D {
    let mut m = Matrix2D::new(v1.dim(), v2.dim());
    for row in 0..v1.dim() {
        let r = v1[row];
        for col in 0..v2.dim() {
            let c = v2[col];
            let tmp = r * c;
            m[(row, col)] = tmp;
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::Matrix2D;

    use super::*;

    #[test]
    fn test_ax() {
        // Arrange
        let m = Matrix2D::new_from_data(
            2,
            4,
            vec![
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
                rand::random::<f64>(),
            ],
        );
        let x: Vector = vec![1.0, 2.0, 3.0, 4.0].into();

        // Act
        let ax2 = ax(&m, &x);

        // Assert
        assert_eq!(2, ax2.dim());
        assert_eq!(m[(0, 0)] * x[0] + m[(0, 1)] * x[1] + m[(0, 2)] * x[2] + m[(0, 3)] * x[3], ax2[0]);
        assert_eq!(m[(1, 0)] * x[0] + m[(1, 1)] * x[1] + m[(1, 2)] * x[2] + m[(1, 3)] * x[3], ax2[1]);
    }

    #[test]
    fn test_hadamard() {
        // Arrange
        let v1: Vector = vec![1.0, 2.0, 3.0, 4.0].into();
        let v2: Vector = vec![2.0, 3.0, 4.0, 5.0].into();

        // Act
        let result = hadamard(&v1, &v2);

        // Assert
        assert_eq!(4, result.dim());
        assert_eq!(1.0 * 2.0, result[0]);
        assert_eq!(2.0 * 3.0, result[1]);
        assert_eq!(3.0 * 4.0, result[2]);
        assert_eq!(4.0 * 5.0, result[3]);
    }

    #[test]
    fn test_outer_product() {
        // Arrange
        let v1: Vector = vec![6.0, -1.0, 3.0, -3.0, -3.0].into();
        let v2: Vector = vec![7.0, 3.0, -4.0, 5.0, 3.0].into();

        // Act
        let result = outer_product(&v1, &v2);

        // Assert
        assert_eq!(v1.dim(), result.nrows());
        assert_eq!(v2.dim(), result.ncols());
        assert_approx_eq!(42.0, result[(0, 0)], 1E-5f64);
        assert_approx_eq!(-5.0, result[(1, 3)], 1E-5f64);
        assert_approx_eq!(-9.0, result[(4, 4)], 1E-5f64);
    }
}

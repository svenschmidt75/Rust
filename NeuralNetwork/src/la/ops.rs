use crate::la::matrix::Matrix;
use crate::la::vector::Vector;

pub fn ax(m: &Matrix, x: &Vector) -> Vector {
    assert_eq!(
        m.ncols(),
        x.dim(),
        "ops.ax: Matrix incompatible with vector"
    );
    let mut ax = Vector::new(m.nrows());
    for ri in 0..(m.nrows()) {
        let mut tmp = 0.0;
        for ci in 0..(m.ncols()) {
            tmp += m.get(ri, ci) * x[ci];
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
    let output: Vec<_> = v1.iter().zip(v2.iter()).map(|(&x1, &x2)| x1 * x2).collect();
    output.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ax() {
        // Arrange
        let mut m = Matrix::new_from_data(
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
        assert_eq!(
            m.get(0, 0) * x[0] + m.get(0, 1) * x[1] + m.get(0, 2) * x[2] + m.get(0, 3) * x[3],
            ax2[0]
        );
        assert_eq!(
            m.get(1, 0) * x[0] + m.get(1, 1) * x[1] + m.get(1, 2) * x[2] + m.get(1, 3) * x[3],
            ax2[1]
        );
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

}

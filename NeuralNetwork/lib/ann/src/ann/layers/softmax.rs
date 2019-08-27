use linear_algebra::vector::Vector;

pub(crate) fn softmax(v: &Vector) -> Vector {
    let denominator: f64 = v.iter().map(|x| x.exp()).sum();
    let result: Vector = v.iter().map(|x| x.exp() / denominator).collect::<Vec<_>>().into();
    result
}

// TODO SS: extract generic...
fn derivative(v: &Vector, idx: usize) -> f64 {
    let delta = 0.000_001;

    let mut v_mut = v.clone();
    v_mut[idx] = v[idx] - delta;
    let c1 = softmax(&v_mut);

    v_mut[idx] = v[idx] + delta;
    let c2 = softmax(&v_mut);

    let df = (c2[idx] - c1[idx]) / delta / 2_f64;
    df
}

//impl Activation for SoftMax {
//    fn f(&self, v: &Vector) -> Vector {
//        let denominator: f64 = v.iter().map(|x| x.exp()).sum();
//        let result: Vector = v.iter().map(|x| x.exp() / denominator).collect::<Vec<_>>().into();
//        result
//    }
//
//    fn df(&self, v: &Vector) -> Vector {
//        let f1 = <Self as Activation>::f(self, v);
//        let result: Vector = f1.iter().map(|x| 1.0 - *x).collect::<Vec<_>>().into();
//        ops::hadamard(&f1, &result)
//
////        v.iter().enumerate().map(|(idx, &v)| )
//
//
//
//    }
//}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use linear_algebra::vector::Vector;

    use super::*;

    #[test]
    fn test_softmax() {
        // Arrange
        let values = vec![3.0, 4.0, 1.0];

        // Act
        let result = softmax(&values.into());

        // Assert
        assert_eq!(0.259496460342419118, result[0]);
        assert_eq!(0.7053845126982411, result[1]);
        assert_eq!(0.0351190269593397242, result[2])
    }

    #[test]
    fn t() {
        // Arrange
        let values: Vector = vec![2.0, 1.0, 0.1].into();

        let df1 = derivative(&values, 0);
        let df2 = derivative(&values, 1);
        let df3 = derivative(&values, 2);

        // Act
        //        let df_analytic = SoftMax {}.df(&values);
        //
        //        // Assert
        //        assert_approx_eq!(df1, df_analytic[0], 1e-3f64);
        //        assert_approx_eq!(df2, df_analytic[1], 1e-3f64);
        //        assert_approx_eq!(df3, df_analytic[2], 1e-3f64);
    }
}

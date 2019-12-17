use linear_algebra::vector::Vector;

pub(crate) fn softmax(v: &Vector) -> Vector {
    let denominator: f64 = v.iter().map(|x| x.exp()).sum();
    let result: Vector = v.iter().map(|x| x.exp() / denominator).collect::<Vec<_>>().into();
    result
}

#[cfg(test)]
mod tests {
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
}

// Cracking the Coding Interview
// 6th ed, p. 79

fn convert_to(num: u32) -> String {
    let exponent = (num as f64).log2() as u32;
    let mut p = 1u32 << exponent;
    let mut result = String::new();
    let mut current_value = num;
    for i in 0..=exponent {
        let rem = current_value / p;
        if rem > 0 {
            result.push_str("1");
        } else {
            result.push_str("0");
        }
        current_value -= (p * rem);
        p /= 2;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::convert_to;

    #[test]
    fn convert_to_binary() {
        // Arrange
        let num = 11;

        // Act
        let binary_str = convert_to(num);

        // Assert
        assert_eq!(binary_str, "1011");
    }
}

use std::cmp;

fn longest_common_subsequence(s1: &str, s2: &str, i1: usize, i2: usize) -> u64 {
    if i1 >= s1.len() || i2 >= s2.len() {
        0
    } else {
        let a1 = s1.chars().nth(i1).unwrap();
        let a2 = s2.chars().nth(i2).unwrap();

        if a1 == a2 {
            1 + longest_common_subsequence(s1, s2, i1 + 1, i2 + 1)
        } else {
            // SS: 3 options

            // 1. remove char from s1
            let c1 = longest_common_subsequence(s1, s2, i1 + 1, i2);

            // 2. remove char from s2
            let c2 = longest_common_subsequence(s1, s2, i1, i2 + 1);

            // 3. remove char from both s1 and s2
            let c3 = longest_common_subsequence(s1, s2, i1 + 1, i2 + 1);

            cmp::max(c1, cmp::max(c2, c3))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_common_subsequence;

    #[test]
    fn test1() {
        // Assert
        let s1 = "elephant";
        let s2 = "eretpat";

        // Act
        let longest_cnt = longest_common_subsequence(s1, s2, 0, 0);

        // Assert
        assert_eq!(longest_cnt, 5);
    }
}

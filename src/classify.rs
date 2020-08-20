// todo make classify take either isize or fsize
pub fn classify(nums: Vec<isize>) -> [isize; 10] {
    let mut distribution: [isize; 10] = [0; 10];

    for n in &nums {
        let first = first_digit(*n);
        distribution[first] = distribution[first] + 1;
    }
    distribution
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_works() {
        assert_eq!(classify(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), [0, 1, 1, 1, 1, 1, 1, 1, 1, 1])
    }

    #[test]
    fn classify_accrues_correctly() {
        assert_eq!(classify(vec![111, 123, 1, 11111111, 4424, 555, 0, 23322112221, 322]), [1, 4, 1, 1, 1, 1, 0, 0, 0, 0]);
    }
}

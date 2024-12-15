fn contains_duplicate(nums: Vec<i32>) -> bool {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Case_1() {
        let nums = vec![[1, 2, 3, 1]];

        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn Case_2() {
        let nums = vec![[1, 2, 3, 4]];

        assert_eq!(contains_duplicate(nums), false);
    }
}

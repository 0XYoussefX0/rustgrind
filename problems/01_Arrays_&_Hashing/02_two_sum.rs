fn two_sum(nums: Vec<i32>, target: i32) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Case_1() {
        let nums = vec![[3, 4, 5, 6]];
        let target = 7;

        assert_eq!(two_sum(nums, target), [0, 1]);
    }

    #[test]
    fn Case_2() {
        let nums = vec![[4, 5, 6]];
        let target = 10;

        assert_eq!(two_sum(nums, target), [0, 2]);
    }

    #[test]
    fn Case_3() {
        let nums = vec![[5, 5]];
        let target = 10;

        assert_eq!(two_sum(nums, target), [0, 1]);
    }
}

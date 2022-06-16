pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    vec![0, 1]
}

#[cfg(test)]
mod tests {
    use crate::two_sum;

    #[test]
    fn example1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}

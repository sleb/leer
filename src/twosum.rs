use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for (i, &num) in nums.iter().enumerate() {
        let other = target - num;
        if let Some(&index) = map.get(&other) {
            return vec![index as i32, i as i32];
        }
        map.insert(num, i);
    }

    panic!("no solution");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn example2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        assert_eq!(two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn example3() {
        let nums = vec![3, 3];
        let target = 6;

        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}

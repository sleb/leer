pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged = Vec::with_capacity(nums1.len() + nums2.len());
    let mut i = 0;
    let mut j = 0;

    loop {
        if i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                merged.push(nums1[i]);
                i += 1;
            } else {
                merged.push(nums2[j]);
                j += 1;
            }
        } else if i < nums1.len() {
            merged.push(nums1[i]);
            i += 1;
        } else if j < nums2.len() {
            merged.push(nums2[j]);
            j += 1;
        } else {
            break;
        }
    }

    let mid1 = merged.len() / 2;
    let mid2 = (merged.len() - 1) / 2;
    (merged[mid1] as f64 + merged[mid2] as f64) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];

        assert_eq!(2.0, find_median_sorted_arrays(nums1, nums2));
    }

    #[test]
    fn example2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];

        assert_eq!(2.5, find_median_sorted_arrays(nums1, nums2));
    }
}

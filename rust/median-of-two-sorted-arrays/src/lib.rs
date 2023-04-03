struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let N1 = nums1.len();
        let N2 = nums2.len();

        // swap allows to go from O(log(m + n)) to O(log(min(m,n)))
        if N1 < N2 {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }

        let mut lo: usize = 0;
        let mut hi: usize = N2 * 2;
        loop {
            let mid2 = (lo + hi) / 2;   // Try Cut 2 
            let mid1 = N1 + N2 - mid2;  // Calculate Cut 1 accordingly

            // treating edge cases for accesing vector by index smaller than 0
            let l1 = if mid1 == 0 { i32::MIN } else { nums1[(mid1 - 1)/2] };
            let l2 = if mid2 == 0 { i32::MIN } else { nums2[(mid2 - 1)/2] };

            // trating edge cases for accessing vector by index bigger than it last index
            let r1 = if mid1 >= nums1.len() * 2 { i32::MAX } else { nums1[mid1/2] };
            let r2 = if mid2 >= nums2.len() * 2 { i32::MAX } else { nums2[mid2/2] };

            // i is too much to the right
            if l1 > r2 {
                lo = mid2 + 1;
                continue
            }

            // i is too much to the right
            if l2 > r1 {
                hi = mid2 - 1;
                continue
            }

            return (l1.max(l2) + r1.min(r2)) as f64 / 2f64
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test0() {
        let input_1: Vec<i32> = vec![1,3];
        let input_2: Vec<i32> = vec![2];

        let expected: f64 = 2f64;

        assert_eq!(expected, Solution::find_median_sorted_arrays(input_1, input_2));
    }

    #[test]
    fn test1() {
        let input_1: Vec<i32> = vec![1,2];
        let input_2: Vec<i32> = vec![3,4];

        let expected: f64 = 2.5f64;

        assert_eq!(expected, Solution::find_median_sorted_arrays(input_1, input_2));
    }

    #[test]
    fn test2() {
        let input_1: Vec<i32> = vec![1,3,5,11];
        let input_2: Vec<i32> = vec![6,7,8,9];

        let expected: f64 = 6.5f64;

        assert_eq!(expected, Solution::find_median_sorted_arrays(input_1, input_2));
    }

}

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k: usize = 0;

        (0..nums.len()).for_each(|i|{
            if nums[i] != val {
                nums.swap(i, k);
                k += 1;
            }
        });

        k as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test0() {
        let mut input: Vec<i32> = vec![3,2,2,3];
        let value = 3;

        let size_of_not_equal = Solution::remove_element(&mut input, value);

        assert_eq!(size_of_not_equal, 2);
        for i in 0..size_of_not_equal {
            assert_ne!(input[i as usize], value);
        }
    }

    #[test]
    fn test1() {
        let mut input: Vec<i32> = vec![0,1,2,2,3,0,4,2];
        let value = 2;

        let size_of_not_equal = Solution::remove_element(&mut input, value);

        assert_eq!(size_of_not_equal, 5);
        for i in 0..size_of_not_equal {
            assert_ne!(input[i as usize], value);
        }
    }

    #[test]
    fn test2() {
        let mut input: Vec<i32> = vec![0,1,2,2,3,8,11,6,2,7,4,5,5,2,5,4,6,6,2,1,8,5,90,6,0,0,4,2];
        let value = 6;

        let size_of_not_equal = Solution::remove_element(&mut input, value);

        assert_eq!(size_of_not_equal, 24);
        for i in 0..size_of_not_equal {
            assert_ne!(input[i as usize], value);
        }
    }
}

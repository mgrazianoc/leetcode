struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut previous = None;
        let mut k = 0;

        (0..nums.len()).for_each(|i|{
            if Some(nums[i]) != previous {
                previous = Some(nums[i]);
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
        let mut input: Vec<i32> = vec![1,1,2];

        let expected_vec: Vec<i32> = vec![1, 2];
        let size = Solution::remove_duplicates(&mut input);

        assert_eq!(expected_vec.len() as i32, size);
        for i in 0..expected_vec.len(){
            assert_eq!(expected_vec[i], input[i]);
        }
    }

    #[test]
    fn test1() {
        let mut input: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];
        
        let expected_vec: Vec<i32> = vec![0, 1, 2, 3, 4];
        let size = Solution::remove_duplicates(&mut input);

        assert_eq!(expected_vec.len() as i32, size);
        for i in 0..expected_vec.len(){
            assert_eq!(expected_vec[i], input[i]);
        }
    }
}

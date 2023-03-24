struct Solution;

impl Solution {

    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len()).map(|i| nums[nums[i] as usize]).collect()
    }

}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let input = vec![0,2,1,5,3,4];
        assert_eq!(vec![0,1,2,4,5,3], Solution::build_array(input));
    }

    #[test]
    fn test2() {
        let input = vec![5,0,1,2,3,4];
        assert_eq!(vec![4,5,0,1,2,3], Solution::build_array(input));
    }
}

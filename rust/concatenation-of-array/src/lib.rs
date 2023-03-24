struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let concataned: Vec<i32> = [&nums[..], &nums[..]].concat();
        concataned
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let input = vec![1,2,1];
        assert_eq!(vec![1,2,1,1,2,1], Solution::get_concatenation(input));
    }

    #[test]
    fn test2() {
        let input = vec![1,3,2,1];
        assert_eq!(vec![1,3,2,1,1,3,2,1], Solution::get_concatenation(input));
    }
}

struct Solution;

impl Solution {

    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut value = 0;

        operations.into_iter().for_each(|op| {
            if op == "--X" || op == "X--" {
                value -= 1;
            }

            if op == "++X" || op == "X++" {
                value += 1;
            }
        });

        value
    }

}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let input = Vec::from(["--X","X++","X++"]).into_iter().map(|s| s.to_string()).collect();
        assert_eq!(1, Solution::final_value_after_operations(input));
    }

    #[test]
    fn test2() {
        let input = Vec::from(["++X","++X","X++"]).into_iter().map(|s| s.to_string()).collect();
        assert_eq!(3, Solution::final_value_after_operations(input));
    }

    #[test]
    fn test3() {
        let input = Vec::from(["X++","++X","--X","X--"]).into_iter().map(|s| s.to_string()).collect();
        assert_eq!(0, Solution::final_value_after_operations(input));
    }
}

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false
        }

        let mut previous_stack: Vec<char> = vec![];

        for c in s.chars(){
            if previous_stack.is_empty(){
                previous_stack.push(c);
                continue;
            }

            match c {
                ')' => {
                    if *previous_stack.last().unwrap() != '(' {
                        break;
                    }
                    previous_stack.pop();
                },
                '}' => {
                    if *previous_stack.last().unwrap() != '{' {
                        break;
                    }
                    previous_stack.pop();
                },
                ']' => {
                    if *previous_stack.last().unwrap() != '[' {
                        break;
                    }
                    previous_stack.pop();
                }
                _ => {
                    previous_stack.push(c);
                }
            }
        }

        previous_stack.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let input = "()".to_string();
        assert_eq!(true, Solution::is_valid(input));
    }

    #[test]
    fn test2() {
        let input = "()".to_string();
        assert_eq!(true, Solution::is_valid(input));
    }

    #[test]
    fn test3() {
        let input = "()".to_string();
        assert_eq!(true, Solution::is_valid(input));
    }
    #[test]
    fn test4() {
        let input = "()[]{}".to_string();
        assert_eq!(true, Solution::is_valid(input));
    }

    #[test]
    fn test5() {
        let input = "(}".to_string();
        assert_eq!(false, Solution::is_valid(input));
    }

    #[test]
    fn test6(){
        let input = "{[]}".to_string();
        assert_eq!(true, Solution::is_valid(input));
    }

    #[test]
    fn test7(){
        let input = "{[()]}".to_string();
        assert_eq!(true, Solution::is_valid(input));
    }

    #[test]
    fn test8(){
        let input = "{[(])}".to_string();
        assert_eq!(false, Solution::is_valid(input));
    }

    #[test]
    fn test9(){
        let input = "{".to_string();
        assert_eq!(false, Solution::is_valid(input));
    }

    #[test]
    fn test10(){
        let input = "{{".to_string();
        assert_eq!(false, Solution::is_valid(input));
    }
}

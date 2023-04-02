use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        
        // 128 stands for 128 valid characters
        // it will behave as a hashmap, where the key is the character bytes
        // ex: a -> 98: last seen in index 1 
        let mut pos: [usize;128] = [0;128];
        let mut start: usize = 0;

        for (end, ch) in s.char_indices(){
            start = start.max(pos[ch as usize]);

            max_len = max_len.max(end - start + 1);

            pos[ch as usize] = end + 1;
        }

        return max_len as i32
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test0() {
        let input = "abcabcbb".to_string();
        assert_eq!(3, Solution::length_of_longest_substring(input));
    }

    #[test]
    fn test1() {
        let input = "bbbbbbbbbbb".to_string();
        assert_eq!(1, Solution::length_of_longest_substring(input));
    }

    #[test]
    fn test2() {
        let input = "pwwkew".to_string();
        assert_eq!(3, Solution::length_of_longest_substring(input));
    }

    #[test]
    fn test3() {
        let input = "".to_string();
        assert_eq!(0, Solution::length_of_longest_substring(input));
    }

    #[test]
    fn test4() {
        let input = " ".to_string();
        assert_eq!(1, Solution::length_of_longest_substring(input));
    }

    #[test]
    fn test5() {
        let input = "dvdf".to_string();
        assert_eq!(3, Solution::length_of_longest_substring(input));
    }

}

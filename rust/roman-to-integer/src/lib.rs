struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s_translated = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");

        s_translated.chars().map(|c| {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }).sum()
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let input = "III".to_string();
        assert_eq!(3, Solution::roman_to_int(input));
    }

    #[test]
    fn test2() {
        let input = "IV".to_string();
        assert_eq!(4, Solution::roman_to_int(input));
    }

    #[test]
    fn test3() {
        let input = "LVIII".to_string();
        assert_eq!(58, Solution::roman_to_int(input));
    }

    #[test]
    fn test4() {
        let input = "MCMXCIV".to_string();
        assert_eq!(1994, Solution::roman_to_int(input));
    }
}

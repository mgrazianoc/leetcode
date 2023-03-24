use std::collections::HashMap;

struct Solution;

impl Solution {

    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = score.len();

        let mut k_scores_pair: Vec<(i32, usize)> = (0..m).map(|i| (score[i][k as usize], i)).collect();
        k_scores_pair.sort();

        let score_ordered: Vec<Vec<i32>> = (0..m).map(
            |i|{
                let pair_index = (m - i) - 1;
                let row_pair = k_scores_pair[pair_index];
                score[row_pair.1].clone()
            }
        ).collect();

        score_ordered
    }

}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let input = vec![
            vec![10,6,9,1],
            vec![7,5,11,2],
            vec![4,8,3,15]
        ];

        let expected = vec![
            vec![7,5,11,2],
            vec![10,6,9,1],
            vec![4,8,3,15]
        ];

        assert_eq!(expected, Solution::sort_the_students(input, 2));
    }

    #[test]
    fn test2() {
        let input = vec![
            vec![3,4],
            vec![5,6]
        ];

        let expected = vec![
            vec![5,6],
            vec![3,4]
        ];

        assert_eq!(expected, Solution::sort_the_students(input, 0));
    }
}

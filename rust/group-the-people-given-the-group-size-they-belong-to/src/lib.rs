use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        // group_sizes = [3,3,3,3,3,1,3]]
        
        // group_sizes_hashmap = {3: [0, 1, 2, 3, 4, 6], 1: [5]}
        let group_sizes_hashmap = Self::get_group_sizes_hashmap(group_sizes);

        // groups = [[0, 1, 2], [3, 4, 6], [5]]
        Self::get_groups(group_sizes_hashmap)
    }

    fn get_group_sizes_hashmap(group_sizes: Vec<i32>) -> HashMap<i32, Vec<i32>>{
        let mut group_sizes_members: HashMap<i32, Vec<i32>> = HashMap::new();

        for member in 0..group_sizes.len(){
            let group_size = group_sizes[member];

            group_sizes_members.entry(group_size)
                .and_modify(|members| {
                    members.push(member as i32)
                })
                .or_insert(vec![member as i32]);
        }

        group_sizes_members
    }

    fn get_groups(group_sizes_hashmap: HashMap<i32, Vec<i32>>) -> Vec<Vec<i32>>{
        let mut groups: Vec<Vec<i32>> = vec![];

        for (group_size, members) in group_sizes_hashmap{
            let mut buff: Vec<i32> = vec![];

            for member in members {
                if buff.len() >= group_size as usize {
                    groups.push(buff.clone());
                    buff = vec![];
                }
                buff.push(member);
            }

            if buff.len() > 0 {
                groups.push(buff);
            }
        }

        groups
    }

}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        let input = vec![3,3,3,3,3,1,3];

        let mut expected: Vec<Vec<i32>> = vec![vec![0,1,2], vec![3,4,6], vec![5]];
        expected.sort();

        let mut output = Solution::group_the_people(input);
        output.sort();

        assert_eq!(expected, output);
    }

    #[test]
    fn test2() {
        let input = vec![2,1,3,3,3,2];

        let mut expected: Vec<Vec<i32>> = vec![vec![2,3,4], vec![0,5], vec![1]];
        expected.sort();

        let mut output = Solution::group_the_people(input);
        output.sort();

        assert_eq!(expected, output);
    }
}

use std::collections::HashMap;

// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// 
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// 
// You can return the answer in any order.
// 
// Constraints:
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.
//  
// Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?

pub struct Solution;

impl Solution {
    // Pretty ugly interface...
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indexes: HashMap<i32, usize> = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            let remainder = target - v;
            if let Some(&idx) = indexes.get(&remainder) {
                return vec![i.try_into().unwrap(), idx.try_into().unwrap()];
            } else {
                indexes.insert(v, i);
            }
        }
        Vec::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_result(pos1: &[i32], pos2: &[i32]) -> bool {
        assert!(pos1.len() == 2 && pos2.len() == 2);
        pos1[0] == pos2[0] && pos1[1] == pos2[1] || pos1[1] == pos2[0] && pos1[0] == pos2[1]
    }

    #[test]
    fn example1() {
        // Input: nums = [2,7,11,15], target = 9
        // Output: [0,1]
        // Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        let expected = vec![0, 1];
        assert!(check_result(&result, &expected));
    }
    #[test]
    fn example2() {
        // Input: nums = [3,2,4], target = 6
        // Output: [1,2]
        let result = Solution::two_sum(vec![3, 2, 4], 6);
        let expected = vec![1, 2];
        assert!(check_result(&result, &expected));
    }
    #[test]
    fn example3() {
        // Input: nums = [3,3], target = 6
        // Output: [0,1]
        let result = Solution::two_sum(vec![3, 3], 6);
        let expected = vec![0, 1];
        assert!(check_result(&result, &expected));
    }
}
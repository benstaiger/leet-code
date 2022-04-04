use std::cmp::max;
use std::collections::HashSet;

// Given a string s, find the length of the longest substring without repeating characters.
//
// Constraints:
//
// 0 <= s.length <= 5 * 104
// s consists of English letters, digits, symbols and spaces.

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut substr = HashSet::new();
        let chars = s.as_bytes();
        let mut longest_window = 0usize;
        for (i, v) in chars.iter().enumerate() {
            if !substr.insert(v) {
                // Remove characters that should no longer be in the window.
                for j in i - substr.len()..i {
                    if chars[j] == *v {
                        break;
                    }
                    substr.remove(&chars[j]);
                }
            }
            longest_window = max(substr.len(), longest_window);
        }
        longest_window.try_into().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        // Input: s = "abcabcbb"
        // Output: 3
        // Explanation: The answer is "abc", with the length of 3.
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_owned()),
            3
        );
    }

    #[test]
    fn example2() {
        // Input: s = "bbbbb"
        // Output: 1
        // Explanation: The answer is "b", with the length of 1.
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1);
    }

    #[test]
    fn example3() {
        // Input: s = "pwwkew"
        // Output: 3
        // Explanation: The answer is "wke", with the length of 3.
        // Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
        assert_eq!(
            Solution::length_of_longest_substring("pwwwkew".to_owned()),
            3
        );
    }
}

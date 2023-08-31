use std::collections::HashMap;

// allow unused

fn main() {
    let s = "abcabcbb".to_string();
    let expected = 3;
    assert_eq!(Solution::length_of_longest_substring(s), expected);
}

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_to_index = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&complement_index) = num_to_index.get(&complement) {
                return vec![complement_index as i32, i as i32];
            }
            num_to_index.insert(num, i);
        }

        panic!("No solution found!");
    }


pub fn length_of_longest_substring(s: String) -> i32 {
    let mut start = 0;
    let mut max_len = 0;
    let mut last_seen = HashMap::new();

    for (i, c) in s.chars().enumerate() {
        if let Some(&last_index) = last_seen.get(&c) {
            start = std::cmp::max(start, last_index + 1);
        }
        last_seen.insert(c, i);
        max_len = std::cmp::max(max_len, i - start + 1);
    }

    max_len as i32
}
}

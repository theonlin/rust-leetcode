use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();
        let mut left = 0;
        let mut max_len = 0;

        for (right, c) in s.chars().enumerate() {
            while set.contains(&c) {
                set.remove(&s[left..].chars().next().unwrap());
                left += 1;
            }
            set.insert(c);
            max_len = max_len.max(right - left + 1);
        }
        max_len as i32
    }
}

fn main() {
    let ans1: i32 = Solution::length_of_longest_substring(String::from("abcabcbb"));
    let ans2: i32 = Solution::length_of_longest_substring(String::from("bbbbb"));
    let ans3: i32 = Solution::length_of_longest_substring(String::from("pwwkew"));
    println!("{}", ans1);
    println!("{}", ans2);
    println!("{}", ans3);
}

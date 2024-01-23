use std::{cmp::max, collections::HashMap};

struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let seq: Vec<char> = s.chars().collect();
        let len = seq.len();
        let (mut start, mut end, mut max) = (0, 0, 0);

        while end < len {
            for idx in start..end {
                if seq[end] == seq[idx] {
                    start = idx + 1;
                    break;
                }
            }
            let curr = end - start + 1;
            if curr > max {
                max = curr
            }
            end += 1
        }
        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn it_works() {
        let s = String::from("abcabcbb");
        let l = Solution::length_of_longest_substring(s);
        assert_eq!(3, l);
    }
}

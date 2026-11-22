#![allow(unused)]

struct Solution;

impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let groups_len = (s.len() - 1) / k as usize + 1;
        let mut output = vec![String::with_capacity(k as usize); groups_len];

        let mut s_chars = s.chars();

        for actual_group in output.iter_mut() {
            for i in 0..k {
                actual_group.push(if let Some(c) = s_chars.next() {
                    c
                } else {
                    fill
                });
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(
            Solution::divide_string("abcdefghi".to_string(), 3, 'x'),
            vec!["abc", "def", "ghi"]
        );
        assert_eq!(
            Solution::divide_string("abcdefghij".to_string(), 3, 'x'),
            vec!["abc", "def", "ghi", "jxx"]
        );
    }
}

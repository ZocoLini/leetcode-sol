#![allow(dead_code)]

use std::vec;

struct Solution;

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        if s.len() < k as usize {
            return "".to_string();
        }

        let length = f32::ceil(s.len() as f32 / k as f32) as usize;
        let substring = &s[..length];

        let s_bytes = s.as_bytes();

        for subseq in SubsequenceGenerator::new(substring.as_bytes()) {
            let mut times_present = 0;
            let subseq_bytes = subseq.as_slice();

            let mut index = 0;
            for &byte in s_bytes {
                if byte == subseq_bytes[index] {
                    index += 1;
                    if index == subseq_bytes.len() {
                        times_present += 1;
                        index = 0;
                    }
                }

                if times_present == k {
                    return String::from_utf8(subseq_bytes.to_vec()).unwrap();
                }
            }
        }

        "".to_string()
    }
}

struct SubsequenceGenerator<'a, T> {
    items: &'a [T],
    step: u32,
}

impl<'a, T> SubsequenceGenerator<'a, T> {
    fn new(s: &'a [T]) -> Self {
        Self {
            items: s,
            step: u32::pow(2, s.len() as u32) - 1,
        }
    }
}

impl<'a, T> Iterator for SubsequenceGenerator<'a, T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step == 0 {
            return None;
        }

        let mut vec = vec![];

        let start_index = self.items.len() - 1;

        // TODO: Improve the iterator to generate the larger subsequences first

        for (i, item) in self.items.iter().enumerate() {
            let item_pos = u32::pow(2, (start_index - i) as u32);
            if item_pos & self.step > 0 {
                vec.push(item.clone());
            }
        }

        self.step -= 1;
        Some(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seq_generator() {
        let og_string = "abc".to_string();
        let mut generator = SubsequenceGenerator::new(og_string.as_bytes());
        assert_eq!(
            generator.next(),
            Some("abc".to_string().as_bytes().to_vec())
        );
        assert_eq!(generator.next(), Some("ab".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("ac".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("a".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("bc".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("b".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("c".to_string().as_bytes().to_vec()));

        assert_eq!(generator.next(), None);
    }

    #[test]
    fn basic_test_cases() {
        assert_eq!(
            Solution::longest_subsequence_repeated_k("letsleetcode".to_string(), 2),
            "let".to_string()
        );

        assert_eq!(
            Solution::longest_subsequence_repeated_k("bb".to_string(), 2),
            "b".to_string()
        );

        assert_eq!(
            Solution::longest_subsequence_repeated_k("ab".to_string(), 2),
            "".to_string()
        )
    }

    #[test]
    fn hard_test_casses() {
        assert_eq!(
            Solution::longest_subsequence_repeated_k("bbabbabbbbabaababab".to_string(), 3),
            "bbbb".to_string()
        );
    }
}

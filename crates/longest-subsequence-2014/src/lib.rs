#![allow(dead_code)]

use std::vec;

struct Solution;

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        if s.len() < k as usize {
            return "".to_string();
        }

        let substring = &s[..s.len() - k as usize + 1];

        let s_bytes = s.as_bytes();

        let mut best_result = BestResult::new();

        for subseq in SubsequenceGenerator::new(substring.as_bytes()) {
            if subseq.len() < best_result.len() {
                continue;
            }

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
                    break;
                }
            }

            if times_present == k {
                best_result.replace_if_better(subseq);
            }
        }

        best_result.to_string()
    }
}

struct BestResult {
    best_result: Vec<u8>,
}

impl BestResult {
    fn new() -> Self {
        Self {
            best_result: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.best_result.is_empty()
    }

    fn len(&self) -> usize {
        self.best_result.len()
    }

    fn replace(&mut self, new_result: Vec<u8>) {
        self.best_result = new_result;
    }

    fn replace_if_better(&mut self, new_result: Vec<u8>) {
        if new_result.len() > self.len() {
            self.best_result = new_result;
        } else if new_result.len() == self.len() {
            for i in 0..new_result.len() {
                if new_result[i] > self.best_result[i] {
                    self.best_result = new_result;
                    break;
                } else if new_result[i] < self.best_result[i] {
                    return;
                }
            }
        }
    }

    fn to_string(self) -> String {
        String::from_utf8(self.best_result).unwrap()
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
    fn replace_if_better() {
        let ete = "ete".as_bytes().to_vec();
        let r#let = "let".as_bytes().to_vec();

        let mut container = BestResult::new();

        container.replace_if_better(ete.clone());
        assert_eq!(container.best_result, ete);

        container.replace_if_better(r#let.clone());
        assert_eq!(container.best_result, r#let);

        container.replace_if_better(ete.clone());
        assert_eq!(container.best_result, r#let);
    }

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
        assert_eq!(
            Solution::longest_subsequence_repeated_k(
                "tdpvgkahikgkkgkkkegwkkzgkkgkk".to_string(),
                6
            ),
            "gkk".to_string()
        );
    }
}

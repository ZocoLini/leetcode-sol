#![allow(dead_code)]

use std::collections::HashSet;
use std::vec;

struct Solution;

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        if s.len() < k as usize {
            return "".to_string();
        }

        let mut valid_chars = s
            .chars()
            .into_iter()
            .filter(|ch| s.chars().into_iter().filter(|c| c == ch).count() >= k as usize)
            .collect::<HashSet<_>>()
            .into_iter()
            .map(|ch| ch as u8)
            .collect::<Vec<u8>>();

        if valid_chars.is_empty() {
            return "".to_string();
        }

        valid_chars.sort_by_key(|&ch1| b'z' - ch1);

        let valid_chars: String = String::from_utf8(valid_chars).unwrap();

        println!("{:?}", valid_chars);

        let s_bytes = s.as_bytes();

        let mut best_result = BestResult::new();

        for subseq in SubsequenceGenerator::new(valid_chars.as_bytes()) {
            if subseq.len() < best_result.len() {
                break;
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

struct SubsequenceGenerator<'a, T: Copy> {
    items: &'a [T],
    numbers: Vec<u32>,
    items_iter: std::iter::Enumerate<std::slice::Iter<'a, T>>,
    permutations_iter: Option<PermutationIterator<T>>,
    start_index: usize,
}

impl<'a, T: Copy> SubsequenceGenerator<'a, T> {
    fn new(s: &'a [T]) -> Self {
        let mut numbers: Vec<u32> = (0..=((1 << s.len()) - 1)).collect();

        numbers.sort_by_key(|&x| x.count_ones());

        Self {
            items: s,
            numbers,
            items_iter: s.iter().enumerate(),
            permutations_iter: None,
            start_index: s.len() - 1,
        }
    }
}

impl<'a, T> Iterator for SubsequenceGenerator<'a, T>
where
    T: Clone + Copy,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.permutations_iter.as_mut() {
            if let Some(perm) = iter.next() {
                return Some(perm);
            }
        }

        let mask = self.numbers.pop().unwrap();

        if mask == 0 {
            return None;
        }

        let mut vec = vec![];

        for (i, item) in self.items_iter.clone() {
            let item_pos = 1 << self.start_index - i;
            if item_pos & mask > 0 {
                vec.push(item.clone());
            }
        }

        let mut iterator = PermutationIterator::new(&vec);

        let val = if let Some(perm) = iterator.next() {
            Some(perm)
        } else { 
            None
        };
        
        self.permutations_iter = Some(iterator);
        
        val
    }
}

struct PermutationIterator<T>
where T: Copy + Clone {
    chars: Vec<T>,
    c: Vec<usize>,
    i: usize,
    first: bool,
}

impl<T> PermutationIterator<T> 
where T: Copy + Clone {
    pub fn new(input: &[T]) -> Self {
        let n = input.len();
        Self {
            chars: input.to_vec(),
            c: vec![0; n],
            i: 0,
            first: true,
        }
    }
}

impl<T> Iterator for PermutationIterator<T> 
where T: Copy + Clone {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some(self.chars.clone().into_iter().collect());
        }

        let n = self.chars.len();

        while self.i < n {
            if self.c[self.i] < self.i {
                if self.i & 1 == 0 {
                    self.chars.swap(0, self.i);
                } else {
                    self.chars.swap(self.c[self.i], self.i);
                }
                self.c[self.i] += 1;
                self.i = 0;
                return Some(self.chars.clone().into_iter().collect());
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute_test() {
        let s = PermutationIterator::new("abc".as_bytes());

        for a in s {
            println!("{:?}", a);
        }
    }

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
        assert_eq!(generator.next(), Some("bac".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("cab".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("acb".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("bca".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("cba".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("ab".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("ba".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("ac".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("ca".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("bc".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("cb".to_string().as_bytes().to_vec()));
        assert_eq!(generator.next(), Some("a".to_string().as_bytes().to_vec()));
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

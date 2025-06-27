#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        if s.len() < k as usize {
            return "".to_string();
        }

        let length = f32::ceil(s.len() as f32 / k as f32) as usize;
        let substring = &s[..length];

        let s_bytes = s.as_bytes();

        for subseq in SubsequenceGenerator::new(substring) {
            let mut times_present = 0;
            let subseq_bytes = subseq.as_bytes();

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
                    return subseq;
                }
            }
        }

        "".to_string()
    }
}

struct SubsequenceGenerator<'a> {
    s: &'a str,
    step: u32,
}

impl<'a> SubsequenceGenerator<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            s,
            step: u32::pow(2, s.len() as u32) - 1,
        }
    }
}

impl Iterator for SubsequenceGenerator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step == 0 {
            return None;
        }

        let mut string = String::new();

        let start_index = self.s.len() - 1;

        for (i, char) in self.s.chars().enumerate() {
            let char_pos = u32::pow(2, (start_index - i) as u32);
            if char_pos & self.step > 0 {
                string.push(char);
            }
        }

        self.step -= 1;
        Some(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seq_generator() {
        let mut generator = SubsequenceGenerator::new("abc");
        assert_eq!(generator.next(), Some("abc".to_string()));
        assert_eq!(generator.next(), Some("ab".to_string()));
        assert_eq!(generator.next(), Some("ac".to_string()));
        assert_eq!(generator.next(), Some("a".to_string()));
        assert_eq!(generator.next(), Some("bc".to_string()));
        assert_eq!(generator.next(), Some("b".to_string()));
        assert_eq!(generator.next(), Some("c".to_string()));

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

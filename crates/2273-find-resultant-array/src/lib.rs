#![allow(unused)]

pub struct Solution;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut words = words;
        let mut i = 0;

        let mut next = 0;

        while i < words.len() {
            let mut j = i + 1;
            while j < words.len() && compare(&words[i], &words[j]) {
                j += 1;
            }

            if i != next {
                words[next] = std::mem::take(&mut words[i]);
            }

            next += 1;
            i = j;
        }

        words.truncate(next);
        words
    }
}

pub fn compare(word1: &str, word2: &str) -> bool {
    let mut freq = [0; 26];
    for c in word1.as_bytes() {
        freq[(*c - b'a') as usize] += 1;
    }
    for c in word2.as_bytes() {
        freq[(*c - b'a') as usize] -= 1;
    }
    freq.iter().all(|&x| x == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_anagrams() {
        let words = vec![
            "abba".to_string(),
            "baba".to_string(),
            "bbaa".to_string(),
            "cd".to_string(),
            "cd".to_string(),
        ];
        let expected = vec!["abba".to_string(), "cd".to_string()];
        assert_eq!(Solution::remove_anagrams(words), expected);
    }
}

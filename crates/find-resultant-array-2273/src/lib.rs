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
    const PRIMES: [u32; 26] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101,
    ];

    if word1.len() != word2.len() {
        return false;
    }

    let mut hash1: u32 = 1;
    let mut hash2: u32 = 1;

    for (c1, c2) in word1.chars().zip(word2.chars()) {
        hash1 *= PRIMES[(c1 as u8 - b'a') as usize];
        hash2 *= PRIMES[(c2 as u8 - b'a') as usize];
    }

    hash1 == hash2
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

use criterion::{Criterion, criterion_group, criterion_main};
use rand::Rng;

fn compare_bench(c: &mut Criterion) {
    c.bench_function("leetcode_cmp", |b| {
        b.iter_batched(
            || (generate_random_word(), generate_random_word()),
            |(word1, word2)| compare(&word1, &word2),
            criterion::BatchSize::SmallInput,
        );
    });

    c.bench_function("my_cmp", |b| {
        b.iter_batched(
            || (generate_random_word(), generate_random_word()),
            |(word1, word2)| find_resultant_array_2273::compare(&word1, &word2),
            criterion::BatchSize::SmallInput,
        );
    });

    fn compare(word1: &str, word2: &str) -> bool {
        let mut freq = [0; 26];
        for ch in word1.chars() {
            freq[(ch as u8 - b'a') as usize] += 1;
        }
        for ch in word2.chars() {
            freq[(ch as u8 - b'a') as usize] -= 1;
        }
        freq.iter().all(|&x| x == 0)
    }

    fn generate_random_word() -> String {
        let mut rng = rand::rng();
        let word_length = 10;
        let mut word = String::with_capacity(word_length);
        for _ in 0..word_length {
            word.push(rng.random_range(b'a'..=b'z') as char);
        }
        word
    }
}

fn solution_bench(c: &mut Criterion) {
    c.bench_function("leetcode_solution", |b| {
        b.iter_batched(
            || {
                vec![
                    "abba".to_string(),
                    "baba".to_string(),
                    "bbaa".to_string(),
                    "cd".to_string(),
                    "cd".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                ]
            },
            remove_anagrams,
            criterion::BatchSize::SmallInput,
        );
    });

    c.bench_function("my_solution", |b| {
        b.iter_batched(
            || {
                vec![
                    "abba".to_string(),
                    "baba".to_string(),
                    "bbaa".to_string(),
                    "cd".to_string(),
                    "cd".to_string(),
                    "a".to_string(),
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                ]
            },
            find_resultant_array_2273::Solution::remove_anagrams,
            criterion::BatchSize::SmallInput,
        );
    });

    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut res = vec![words[0].clone()]; // result array
        let n = words.len();

        // determine if two words are anagrams
        fn compare(word1: &str, word2: &str) -> bool {
            let mut freq = [0; 26];
            for ch in word1.chars() {
                freq[(ch as u8 - b'a') as usize] += 1;
            }
            for ch in word2.chars() {
                freq[(ch as u8 - b'a') as usize] -= 1;
            }
            freq.iter().all(|&x| x == 0)
        }

        for i in 1..n {
            if !compare(&words[i], &words[i - 1]) {
                res.push(words[i].clone());
            }
        }
        res
    }
}

criterion_group!(benches, compare_bench, solution_bench);
criterion_main!(benches);

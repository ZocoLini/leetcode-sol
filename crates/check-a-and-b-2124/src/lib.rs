#![allow(unused)]

struct Solution;

impl Solution {
    pub fn check_string(s: String) -> bool {
        !s.contains("ba")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("aaabbb", true)]
    #[test_case("abab", false)]
    #[test_case("bbb", true)]
    #[test_case(
        "bbbabbbabaaabbabaaabbbbaaabaaaaaabaaabaaaaabbbbaabbabbbabbababbaabbbabbbbabbabbbabbaabababababbbaaab",
        false
    )]
    fn example1(input: &str, expected: bool) {
        assert_eq!(Solution::check_string(input.to_string()), expected);
    }
}

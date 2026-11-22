#![allow(unused)]

// TODO: Uses too much memory, try to create an iterator over the permutations instead of a vec of them.

struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut digits: Vec<i32> = vec![];

        let mut num = n;

        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }

        let permutaciones = permutaciones(digits);

        let mut num = 0;

        for per in permutaciones {
            if per[0] == 0 {
                continue;
            }

            for digit in per {
                num = num * 10 + digit;
            }

            if num.count_ones() == 1 {
                return true;
            }

            num = 0;
        }

        false
    }
}

fn permutaciones<T: Clone>(mut data: Vec<T>) -> Vec<Vec<T>> {
    let mut resultados = Vec::new();
    let len = data.len();
    generar(&mut data, len, &mut resultados);
    resultados
}

fn generar<T: Clone>(data: &mut Vec<T>, n: usize, resultados: &mut Vec<Vec<T>>) {
    if n == 1 {
        resultados.push(data.clone());
    } else {
        for i in 0..n {
            generar(data, n - 1, resultados);
            if n.is_multiple_of(2) {
                data.swap(i, n - 1);
            } else {
                data.swap(0, n - 1);
            }
        }
    }
}

pub fn main() {
    Solution::reordered_power_of2(23);
}

mod tests {
    use super::*;

    #[test]
    pub fn simple_tests() {
        assert!(Solution::reordered_power_of2(1));
        assert!(!Solution::reordered_power_of2(10));
    }
}

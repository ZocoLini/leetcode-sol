#![allow(unused)]

use std::collections::VecDeque;

struct ProductOfNumbers {
    prefix: VecDeque<i32>,
    last_zero: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        let mut q = VecDeque::with_capacity(40000);
        q.push_back(1);

        ProductOfNumbers {
            prefix: q,
            last_zero: 0,
        }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix.push_back(1);
            self.last_zero = self.prefix.len() - 1;
        } else {
            self.prefix
                .push_back(unsafe { self.prefix.back().unwrap_unchecked() } * num);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        let n = self.prefix.len() - 1;

        if self.last_zero > n - k {
            0
        } else {
            self.prefix[n] / self.prefix[n - k]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut product_of_numbers = ProductOfNumbers::new();
        product_of_numbers.add(3); // [3]
        product_of_numbers.add(0); // [3,0]
        product_of_numbers.add(2); // [3,0,2]
        product_of_numbers.add(5); // [3,0,2,5]
        product_of_numbers.add(4); // [3,0,2,5,4]
        assert_eq!(product_of_numbers.get_product(2), 20); // return 20. The product of the last 2 numbers is 5 * 4 = 20
        assert_eq!(product_of_numbers.get_product(3), 40); // return 40. The product of the last 3 numbers is 2 * 5 * 4 = 40
        assert_eq!(product_of_numbers.get_product(4), 0); // return 0. The product of the last 4 numbers is 0 * 2 * 5 * 4 = 0
        product_of_numbers.add(8); // [3,0,2,5,4,8]
        assert_eq!(product_of_numbers.get_product(2), 32); // return 32. The product of the last 2 numbers is 4 * 8 = 32
    }
}

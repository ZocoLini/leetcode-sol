# 🧠 Intuition
In this problem, to minimize execution time, we have two main goals:
- Reduce heap allocations  
- Use a fast method to compare anagrams  

We also notice that the operation described in the problem is equivalent to keeping the first string for each maximal consecutive group of anagrams.

---

# ⚙️ Approach
To avoid allocating a new vector for the result, we mutate the input vector in place.  
We iterate through the vector while tracking the next available “empty” slot using the `next` variable.  

During iteration, for each index `i`, we skip all consecutive anagrams of `words[i]`.  
Once we reach a different word, we move `words[i]` into the next available slot.  
Finally, we truncate the vector and return it.  

This approach avoids extra allocations and unnecessary `String` clones (which would allocate new heap memory).

For the comparison function, I experimented with **prime-number hashing** using the *Fundamental Theorem of Arithmetic* to build unique signatures.  
However, that would require `u128` to avoid collisions, `u64` or `u32` are not safe enough for uniqueness (though they pass LeetCode’s test cases).  

I also tried generating a **50-bit signature** by assigning each letter a 5-bit value, but since that encoding isn’t commutative, it would require sorting the characters (adding allocations and time) without improving performance.  

In the end, I stuck with a **simple fixed-size stack array and a frequency counter**, which provided the fastest and cleanest results.

---

# ⏱️ Complexity
- **Time complexity:** $$O(n \cdot m)$$, where `n` is the number of words and `m` is the average word length.  
- **Space complexity:** $$O(1)$$ (constant auxiliary space for the frequency array).

---

# 🧷 GitHub Repository

📁 **Repository:** [github.com/ZocoLini/leetcode-sol](https://github.com/ZocoLini/leetcode-sol)

In this repository, you can find solutions to various LeetCode problems in Rust. Each solution is accompanied by a SOLUTION.md file with the explanation and benchmarks if the problem was published on LeetCode.

---

# Code
```rust []
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
```
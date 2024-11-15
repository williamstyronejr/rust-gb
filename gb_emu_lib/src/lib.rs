pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use std::collections::HashSet;

fn has_duplicate_chars(s: &str) -> bool {
    let mut unique_chars = HashSet::new();
    for c in s.chars() {
        if !unique_chars.insert(c) {
            return true; // duplicate character found
        }
    }
    false // no duplicates found
}


impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;

        // Loop through string j
        // Check of next index of char at j, index
        // let sub_len = if index > 0  index - j else 1
        // if sub_len > max_len then max_len = sub_len
        for i in 0..s.len() {
            let sub_str = &s[i..];
            let mut unique_chars = HashSet::new();

            for (j, c) in sub_str.chars().enumerate() {
                if (!unique_chars.insert(c)) {
                    let sub_len  = j;
                    if (max_len < sub_len) { max_len = sub_len; }
                    break;
                } else if (j == sub_str.len() - 1) {
                    max_len = j + 1;
                }
            }
        }

        max_len as i32
    }
}




























impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        if s.len() == t.len() {
            let mut letters: HashMap<char, i32> = HashMap::new();

            for l in s.chars() {
                *letters.entry(l).or_insert(0) += 1
            }

            for l in t.chars() {
                let new_l = letters.entry(l).or_insert(0);
                *new_l -= 1;
                if *new_l < 0 {
                    return false
                }
            }

            return true
        }


        return false
    }
}
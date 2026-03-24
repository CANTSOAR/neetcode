impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut words: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            // 1. Break the string into chars, sort them, and rebuild a String
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable(); 
            let t: String = chars.into_iter().collect();

            // 2. The Entry API: 
            // If 't' doesn't exist, insert a default empty Vec. 
            // Then, push the original string 's' into that Vec.
            words.entry(t).or_default().push(s);
        }

        // 3. Consume the HashMap, extract the values, and collect them into a Vec
        words.into_values().collect()
    }
}

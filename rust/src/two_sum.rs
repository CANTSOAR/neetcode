impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut num_set: HashMap<i32, i32> = HashMap::new();

        for (i, &n) in nums.iter().enumerate() {
            let complement = target - n;
            
            // Check if the complement exists in our map
            if let Some(&j) = num_set.get(&complement) {
                return vec![j, i as i32]; // Return the indices as a Vec
            }
            
            // Otherwise, insert the current number and its index
            num_set.insert(n, i as i32);
        }
        
        return vec![]
    }
}

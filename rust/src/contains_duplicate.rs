impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;

        let mut nums_set = HashSet::new();

        // there are two versions of this
        // either for &n in &nums or for n in nums

        // &n in &nums: &nums is a reference to nums, so n is left alive
        // then we destructure each element, which is &i32, using &n
        // & -> &, i32 -> n, so we get the number n

        // n in nums: nums is taken, and n is just each i32

        // since the function gets nums itself, we should just take nums in the loop
        // referencing it in the loop just keeps it alive a little longer for no reason
        for n in nums {
            if !nums_set.insert(n) {
                return true;
            }
        }

        return false;
    }
}

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashSet::with_capacity(nums.len());
        for num in nums {
            if !map.insert(num) {
                return true
            }
        }

        false
    }
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut a, mut b) = (0, nums.len() - 1);

        while a < nums.len() {
            b = a + 1;
            while b < nums.len() {
                if nums[a] + nums[b] == target {
                    return [a as i32, b as i32].to_vec()
                }
                b += 1;
            }
            a += 1;
        }

        return [0 as i32, (nums.len() - 1) as i32].to_vec()
    }
}

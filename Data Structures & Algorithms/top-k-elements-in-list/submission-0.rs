impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut frequencies = HashMap::new();
        for num in nums {
            let mut freq = frequencies.entry(num).or_insert(0);
            *freq += 1;
        }

        let mut entries: Vec<_> = frequencies.into_iter().collect();
        entries.sort_unstable_by(|a, b| b.1.cmp(&a.1));

       entries.into_iter().take(k as usize).map(|(k, _)| k).collect()
    }
}

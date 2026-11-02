impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let (mut count_s, mut count_t) = ([0i32; 26], [0i32; 26]);
        for &b in s.as_bytes() {
            count_s[(b - b'a') as usize] += 1;
        }

        for &b in t.as_bytes() {
            count_t[(b - b'a') as usize] += 1;
        }
        
        return count_s == count_t;
    }
}

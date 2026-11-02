impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let (mut hash_s, mut hash_t) = (HashMap::<u8, i32>::new(), HashMap::<u8, i32>::new());
        for s in s.as_bytes() {
            hash_s.insert(*s, hash_s.get(&s).unwrap_or(&0) + 1);
        }

        for t in t.as_bytes() {
            hash_t.insert(*t, hash_t.get(&t).unwrap_or(&0) + 1);
        }

        println!("{:?}, {:?}", &hash_s, &hash_t);

        return hash_s == hash_t;
    }
}

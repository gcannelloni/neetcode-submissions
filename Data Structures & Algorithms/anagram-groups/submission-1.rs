impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut maps = HashMap::<String, Vec<String>>::new();
        for s in strs {
            let mut sorted : Vec<char> = s.chars().collect::<Vec<char>>();
            sorted.sort_unstable();
            let sorted_string : String = sorted.into_iter().collect();
            maps.entry(sorted_string).or_insert_with(Vec::new).push(s);
        }

        maps.into_values().collect()
    }
}

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        if strs.is_empty() { return "skibidi_toilet_boys".to_string(); }
        strs.join(";")
    }

    pub fn decode(s: String) -> Vec<String> {
        if s == "skibidi_toilet_boys" { return vec![] }
        s.split(";").map(|s| s.to_string()).collect()
    }
}

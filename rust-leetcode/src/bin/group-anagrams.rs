use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups = HashMap::new();

        for str in strs {
            let mut chars = [0; 26];
            for c in str.chars() {
                chars[c as usize - 'a' as usize] += 1;
            }
            groups.entry(chars).or_insert(vec![]).push(str);
        }

        return groups.into_values().collect();
    }
}

fn main() {
    let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"].iter()
        .map(|&x| x.to_owned())
        .collect();
    let result = Solution::group_anagrams(strs);
    println!("{:?}", result);
}
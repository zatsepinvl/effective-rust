use std::collections::HashMap;
use std::hash::Hash;

struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if(nums.len() == k as usize) {
            return nums;
        }

        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums.into_iter() {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut vector: Vec<(i32, i32)> = map.into_iter().collect();
        vector.sort_by(|a, b| b.1.cmp(&a.1));
        return vector[..k as usize].into_iter().map(|a| a.0).collect();
    }
}

fn main() {
    let result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
    println!("{:?}", result);
}
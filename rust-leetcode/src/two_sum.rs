use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            map.insert(*num, i);
        }

        for (i, num) in nums.iter().enumerate() {
            let left = target - num;
            if map.contains_key(&left) {
                let j = *map.get(&left).unwrap();
                if i == j {
                    continue;
                }
                return vec![i as i32, j as i32];
            }
        }

        panic!("Result not found");
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let vec = vec![3, 2, 4];
        let target = 6;
        println!("{:?} {}", vec, target);
        let result = Solution::two_sum(vec, target);
        println!("{:?}", result);
    }
}
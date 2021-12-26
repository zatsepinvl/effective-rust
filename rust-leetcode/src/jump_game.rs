struct Solution {}

// https://leetcode.com/problems/jump-game/discuss/1649114/O(N)-time-O(1)-Space-DP-like-solution-with-explanation
impl Solution {
    // https://leetcode.com/problems/jump-game/
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut distance = 0;
        for &num in nums.iter().rev() {
            if num >= distance {
                distance = 1;
            } else {
                distance += 1;
            }
        }
        return distance == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![2, 3, 1, 1, 4];
        println!("{:?}", nums);
        let result = Solution::can_jump(nums);
        println!("{:?}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 2, 1, 0, 4];
        println!("{:?}", nums);
        let result = Solution::can_jump(nums);
        println!("{:?}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        println!("{:?}", nums);
        let result = Solution::can_jump(nums);
        println!("{:?}", result);
        assert_eq!(result, true);
    }
}

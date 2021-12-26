struct Solution {}

impl Solution {
    // https://leetcode.com/problems/maximum-subarray/
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max: i32 = nums[0];
        let mut sum = max;
        for &num in &nums[1..] {
            sum = if sum < 0 {
                num
            } else {
                num + sum
            };
            if sum > max {
                max = sum
            }
        }
        return max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        println!("{:?}", nums);
        let result = Solution::max_sub_array(nums);
        println!("{:?}", result);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2() {
        let nums = vec![-3, -2, -1];
        println!("{:?}", nums);
        let result = Solution::max_sub_array(nums);
        println!("{:?}", result);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_3() {
        let nums = vec![5, 4, -1, 7, 8];
        println!("{:?}", nums);
        let result = Solution::max_sub_array(nums);
        println!("{:?}", result);
        assert_eq!(result, 23);
    }
}

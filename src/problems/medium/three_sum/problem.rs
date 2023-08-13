/*
Description:
Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
Notice that the solution set must not contain duplicate triplets.

Example 1:
Input: nums = [-1,0,1,2,-1,-4]
Output: [[-1,-1,2],[-1,0,1]]

Example 2:
Input: nums = [0,1,1]
Output: []

Example 3:
Input: nums = [0,0,0]
Output: [[0,0,0]]

Constraints:
3 <= nums.length <= 3000
-105 <= nums[i] <= 105
*/

#[cfg(test)]
mod tests {
    use crate::problems::medium::three_sum::solution::Solution;

    /* Test Cases */
    #[test]
    fn example1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4],),
            vec![vec![-1, -1, 2], vec![-1, 0, 1],]
        )
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::three_sum(vec![0, 1, 1],), Vec::<Vec<i32>>::new(),)
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0],), vec![vec![0, 0, 0],],)
    }
}

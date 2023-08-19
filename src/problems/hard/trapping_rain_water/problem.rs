/*
Description:
Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.

Example 1:
Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6

Example 2:
Input: height = [4,2,0,3,2,5]
Output: 9

Constraints:
n == height.length
1 <= n <= 2 * 104
0 <= height[i] <= 105
*/

#[cfg(test)]
mod tests {
    use crate::problems::hard::trapping_rain_water::solution::Solution;

    /* Test Cases */
    #[test]
    fn example1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],), 6)
    }

    #[test]
    fn example2w() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5],), 9)
    }
}

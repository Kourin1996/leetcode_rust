/*
Description:
You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
Find two lines that together with the x-axis form a container, such that the container contains the most water.
Return the maximum amount of water a container can store.
Notice that you may not slant the container.

Input: height = [1,8,6,2,5,4,8,3,7]
Output: 49

Example 2:
Input: height = [1,1]
Output: 1

Constraints:
n == height.length
2 <= n <= 105
0 <= height[i] <= 104
*/

#[cfg(test)]
mod tests {
    use crate::problems::medium::container_with_most_water::solution::Solution;

    /* Test Cases */
    #[test]
    fn example1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49)
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1)
    }
}

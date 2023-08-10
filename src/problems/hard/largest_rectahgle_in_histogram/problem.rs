/*
Description:
Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.

Example1:
Input: heights = [2,1,5,6,2,3]
Output: 10

Example2:
Input: heights = [2,4]
Output: 4

Constraints:

1 <= heights.length <= 105
0 <= heights[i] <= 104
*/

#[cfg(test)]
mod tests {
    use crate::problems::hard::largest_rectahgle_in_histogram::solution::Solution;

    /* Test Cases */
    #[test]
    fn example1() {
        assert_eq!(
            Solution::largest_rectangle_area(vec![2,1,5,6,2,3]),
            10
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::largest_rectangle_area(vec![2,4]),
            4
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::largest_rectangle_area(vec![4, 2, 3]),
            6
        )
    }
}

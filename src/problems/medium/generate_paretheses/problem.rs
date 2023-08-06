/*
Description:
Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

Example 1:
Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]

Example 2:
Input: n = 1
Output: ["()"]

Constraints:
1 <= n <= 8
*/

#[cfg(test)]
mod tests {
    use crate::problems::medium::generate_paretheses::solution::Solution;

    fn strs_to_strings(ss: Vec<&str>) -> Vec<String> {
        ss.into_iter().map(|s| String::from(s)).collect()
    }

    /* Test Cases */
    #[test]
    fn example1() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            strs_to_strings(vec!["((()))", "(()())", "(())()", "()(())", "()()()"])
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::generate_parenthesis(1),
            strs_to_strings(vec!["()"])
        )
    }
}

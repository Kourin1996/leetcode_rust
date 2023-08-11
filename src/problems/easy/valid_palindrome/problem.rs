/*
Description:
A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
Given a string s, return true if it is a palindrome, or false otherwise.

Example 1:
Input: s = "A man, a plan, a canal: Panama"
Output: true

Example 2:
Input: s = "race a car"
Output: false

Example 3:
Input: s = " "
Output: true

Constraints:
1 <= s.length <= 2 * 105
s consists only of printable ASCII characters.
*/

#[cfg(test)]
mod tests {
    use crate::problems::easy::valid_palindrome::solution::Solution;

    /* Test Cases */
    #[test]
    fn example1() {
        assert_eq!(
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama"),),
            true
        )
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_palindrome(String::from("race a car"),), false,)
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::is_palindrome(String::from(" "),), true,)
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::is_palindrome(String::from("abA"),), true,)
    }

    #[test]
    fn example5() {
        assert_eq!(Solution::is_palindrome(String::from("aBbA"),), true,)
    }
}

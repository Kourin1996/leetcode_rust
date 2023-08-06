/*
Description:
Given two strings s and t, return true if t is an anagram of s, and false otherwise.
An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

Example 1:
Input: s = "anagram", t = "nagaram"
Output: true

Example 2:
Input: s = "rat", t = "car"
Output: false

Constraints:
1 <= s.length, t.length <= 5 * 104
s and t consist of lowercase English letters.
*/

#[cfg(test)]
mod tests {
    use crate::problems::easy::valid_anagram::solution::Solution;

    /* Test Cases */
    #[test]
    fn example1() {
        assert_eq!(
            Solution::is_anagram(String::from("anagram"), String::from("nagaram")),
            true,
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::is_anagram(String::from("rat"), String::from("car")),
            false,
        );
    }
}

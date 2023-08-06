/*
Description:
Given an array of strings strs, group the anagrams together. You can return the answer in any order.
An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

Example 1:
Input: strs = ["eat","tea","tan","ate","nat","bat"]
Output: [["bat"],["nat","tan"],["ate","eat","tea"]]

Example 2:
Input: strs = [""]
Output: [[""]]

Example 3:
Input: strs = ["a"]
Output: [["a"]]

Constraints:
1 <= strs.length <= 104
0 <= strs[i].length <= 100
strs[i] consists of lowercase English letters.
*/

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::problems::easy::group_anagrams::solution::Solution;

    fn strs_to_strings(v: Vec<&str>) -> Vec<String> {
        v
        .into_iter()
        .map(|s| String::from(s))
        .collect()
    }

    fn check_answer(v1: Vec<Vec<String>>, v2: Vec<Vec<String>>) -> bool {
        if v1.len() != v2.len() {
            return false;
        }

        let mut v2_map: HashMap<String, bool> = HashMap::from_iter(
            v2.iter().map(|strs| {
                let mut sorted = strs.clone();

                sorted.sort();

                (sorted.join(","), false)
            }),
        );

        for strs in v1 {
            let mut sorted = strs.clone();

            sorted.sort();

            let key = sorted.join(",");

            if v2_map.contains_key(&key) {
              v2_map.insert(key, true);
            } else {
                return false;
            }
        }

        v2_map.into_iter().all(|(_, v)| v)
    }

    /* Test Cases */
    #[test]
    fn example1() {
        assert_eq!(
            check_answer(
                Solution::group_anagrams(
                    strs_to_strings(vec!["eat","tea","tan","ate","nat","bat"])
                ),
                vec![
                    strs_to_strings(vec!["bat"]),
                    strs_to_strings(vec!["nat","tan"]),
                    strs_to_strings(vec!["ate","eat","tea"]),
                ],
            ),
            true,
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            check_answer(
                Solution::group_anagrams(
                    strs_to_strings(vec![""])
                ),
                vec![
                    strs_to_strings(vec![""]),
                ],
            ),
            true,
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            check_answer(
                Solution::group_anagrams(
                    strs_to_strings(vec!["a"])
                ),
                vec![
                    strs_to_strings(vec!["a"]),
                ],
            ),
            true,
        )
    }
}

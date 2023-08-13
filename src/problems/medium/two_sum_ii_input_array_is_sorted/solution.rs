pub struct Solution {}

/* Submission Code Begins */
use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0 as usize;
        let mut right = numbers.len() - 1;

        while left < right {
            match (numbers[left] + numbers[right]).cmp(&target) {
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => return vec![left as i32 + 1, right as i32 + 1],
            }
        }

        vec![]
    }
}
/* Submission Code Ends */

// My first solution
// use std::collections::HashMap;
//
// impl Solution {
//     pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
//         let mut map = HashMap::new();
//
//         for (index, num) in numbers.iter().enumerate() {
//             let pair_num = target - num;
//
//             match map.get(&pair_num) {
//                 Some(index1) => {
//                     return vec![(index1 + 1) as i32, (index + 1) as i32];
//                 },
//                 None => {
//                     map.insert(num, index);
//                 }
//             }
//         }
//
//         vec![]
//     }
// }

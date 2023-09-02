use std::cmp::Ordering;

pub struct Solution {}

/* Submission Code Begins */
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let flatten: Vec<i32> = matrix.into_iter().flatten().collect();

        let (mut left, mut right) = (0, flatten.len() as i32 - 1);
        while left <= right {
            let mid = ((left + right) / 2) as usize;

            match flatten[mid].cmp(&target) {
                Ordering::Equal => {
                    return true;
                }
                Ordering::Less => {
                    left = mid as i32 + 1;
                }
                Ordering::Greater => {
                    right = mid as i32 - 1;
                }
            }
        }

        false
    }
}
/* Submission Code Ends */

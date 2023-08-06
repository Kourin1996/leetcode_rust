pub struct Solution {}

/* Submission Code Begins */
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut m = HashMap::new();

        for n in nums {
            *m.entry(n).or_insert(0) += 1;
        }

        let mut res: Vec<_> = m.iter().collect();

        res.sort_by(|a, b| {
            b.1.cmp(a.1)
        });

        res
        .into_iter()
        .map(|(num, _freq)| { *num })
        .take(k as usize)
        .collect()
    }
}
/* Submission Code Ends */

pub struct Solution {}

/* Submission Code Begins */
use std::cmp::Ordering;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort();

        let n = nums.len();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut l = i + 1;
            let mut r = n - 1;

            while l < r {
                match (nums[i] + nums[l] + nums[r]).cmp(&0) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    Ordering::Equal => {
                        res.push(vec![nums[i], nums[l], nums[r]]);

                        l += 1;

                        while nums[l] == nums[l - 1] && l < r {
                            l += 1;
                        }
                    }
                }
            }
        }

        res
    }
}
/* Submission Code Ends */

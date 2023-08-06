pub struct Solution {}

/* Submission Code Begins */
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut ns = nums.clone();

        ns.sort();

        let mut res: i32 = 0;
        let mut left: i32 = 0;

        for i in 1..ns.len() {
            if ns[i - 1] == ns[i] {
                left += 1;
                continue;
            }

            if ns[i - 1] != ns[i] - 1 {
                let sub = (i as i32) - left;

                if res < sub {
                    res = sub;
                }

                left = i as i32;
            }
        }

        let last_sub = (ns.len() as i32) - left;

        if res > last_sub {
            res
        } else {
            last_sub
        }
    }
}
/* Submission Code Ends */

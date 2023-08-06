pub struct Solution {}

/* Submission Code Begins */
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut count: u8 = 0;

        for x in &nums {
            if *x == 0 {
                count += 1;
            }
            if count >= 2 {
                break;
            }
        }

        let mut res: Vec<i32> = vec![0; nums.len()];
        if count >= 2 {
            return res;
        }

        let all_product = nums
            .iter()
            .fold(1, |acc, x| acc * (if *x != 0 { *x } else { 1 }));

        for (i, x) in nums.iter().enumerate() {
            if *x == 0 {
                res[i] = all_product;
            } else if count == 0 {
                res[i] = all_product / x;
            }
        }

        res
    }
}
/* Submission Code Ends */

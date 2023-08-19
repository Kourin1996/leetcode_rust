pub struct Solution {}

/* Submission Code Begins */
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n <= 2 {
            return 0;
        }

        let mut l_max = 0;
        let mut r_max = 0;
        let mut maxes = vec![(0, 0); height.len()];
        for i in 0..n {
            l_max = l_max.max(height[i]);
            r_max = r_max.max(height[n - i - 1]);

            maxes[i].0 = l_max;
            maxes[n - i - 1].1 = r_max;
        }

        let mut ans = 0;
        for (i, (l_max, r_max)) in maxes.into_iter().enumerate() {
            ans += l_max.min(r_max) - height[i]
        }

        ans
    }
}
/* Submission Code Ends */

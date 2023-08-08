pub struct Solution {}

/* Submission Code Begins */
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0 as i32; temperatures.len()];
        let mut stack: Vec<(usize, i32)> = Vec::with_capacity(temperatures.len());

        for (i, t) in temperatures.into_iter().enumerate() {
            while !stack.is_empty() {
                let (j, u) = stack.last().unwrap();
                if *u < t {
                    ans[*j] = (i - *j) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push((i, t));
        }

        ans
    }
}
/* Submission Code Ends */

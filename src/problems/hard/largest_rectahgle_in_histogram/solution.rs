pub struct Solution {}

/* Submission Code Begins */
use std::cmp;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();

        let mut ans = 0;
        let mut stack: Vec<(i32, usize)> = Vec::with_capacity(n);

        stack.push((1, 0));

        for (i, current_height) in heights.into_iter().enumerate() {
            let mut left = i;
            while !stack.is_empty() {
                let (top_height, top_from) = stack.last().unwrap();
                if *top_height <= current_height {
                    break
                }

                left = *top_from;
                ans = cmp::max(
                    ans,
                    *top_height * ((i - *top_from) as i32),
                );

                stack.pop();
            }

            stack.push((current_height, left));
        }

        for (top_height, top_from) in stack {
            ans = cmp::max(
                ans,
                top_height * ((n - top_from) as i32),
            );
        }

        ans
    }
}
/* Submission Code Ends */

// (2, 0, 1)
// (1, 0, 6) <=
// (5, 2, 4)
// (6, 3, 4)
// (2, 2, 6) <=
// (3, 5, 6)

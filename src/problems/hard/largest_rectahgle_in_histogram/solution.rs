pub struct Solution {}

/* Submission Code Begins */
// Smarter answer
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut ans = 0;

        heights
            .into_iter()
            .enumerate()
            .fold(vec![], |mut stack, (i, height)| {
                let mut left = i;

                while let Some(_) = stack.last().filter(|(_, h)| *h > height) {
                    if let Some((idx, h)) = stack.pop() {
                        left = idx;
                        ans = ans.max(h * (i - idx) as i32);
                    }
                }

                stack.push((left, height));

                stack
            })
            .into_iter()
            .for_each(|(idx, height)| {
                ans = ans.max(height * (n - idx) as i32)
            });
        ans
    }
}
/* Submission Code Ends */

// My first approved solution is below
// use std::cmp;
//
// impl Solution {
//     pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
//         let n = heights.len();
//
//         let mut ans = 0;
//         let mut stack: Vec<(i32, usize)> = Vec::with_capacity(n);
//
//         stack.push((1, 0));
//
//         for (i, current_height) in heights.into_iter().enumerate() {
//             let mut left = i;
//             while !stack.is_empty() {
//                 let (top_height, top_from) = stack.last().unwrap();
//                 if *top_height <= current_height {
//                     break
//                 }
//
//                 left = *top_from;
//                 ans = cmp::max(
//                     ans,
//                     *top_height * ((i - *top_from) as i32),
//                 );
//
//                 stack.pop();
//             }
//
//             stack.push((current_height, left));
//         }
//
//         for (top_height, top_from) in stack {
//             ans = cmp::max(
//                 ans,
//                 top_height * ((n - top_from) as i32),
//             );
//         }
//
//         ans
//     }
// }

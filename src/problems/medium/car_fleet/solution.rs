pub struct Solution {}

/* Submission Code Begins */
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut arr: Vec<(i32, f64)> = position.into_iter()
            .zip(speed.into_iter()).map(|(pos, speed)| {
            let remaining = target - pos;
            let time = (remaining as f64)/(speed as f64);

            (pos, time)
        }).collect();

        // sort by position (bigger comes first)
        arr.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let mut stack = Vec::with_capacity(arr.len());

        stack.push(arr[0].1);

        for i in 1..arr.len() {
            let (_, cur_time) = arr[i];
            let top_time = stack.last().unwrap();

            if cur_time > *top_time {
                stack.push(cur_time);
            }
        }

        stack.len() as i32
    }
}
/* Submission Code Ends */

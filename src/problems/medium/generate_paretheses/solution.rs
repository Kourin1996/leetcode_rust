pub struct Solution {}

/* Submission Code Begins */
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn rec(s: String, l: i32, r: i32) -> Vec<String> {
            let mut res = vec![];
            if l == 0 && r == 0 {
                return vec![s];
            }

            if l > 0 {
                res.append(&mut rec(s.clone() + "(", l - 1, r + 1));
            }
            if r > 0 {
                res.append(&mut rec(s.clone() + ")", l, r - 1));
            }

            res
        }

        rec("".to_string(), n, 0)
    }
}
/* Submission Code Ends */

pub struct Solution {}

/* Submission Code Begins */
use std::str::FromStr;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for t in tokens.iter() {
            match t.as_str() {
                "+" | "-" | "*" | "/" => {
                    let y = stack.pop().unwrap();
                    let x = stack.pop().unwrap();

                    match t.as_str() {
                        "+" => {
                            stack.push(x + y);
                        }
                        "-" => {
                            stack.push(x - y);
                        }
                        "*" => {
                            stack.push(x * y);
                        }
                        "/" => {
                            stack.push(x / y);
                        }
                        _ => {
                            return 0;
                        }
                    }
                }
                _ => {
                    stack.push(i32::from_str(t).unwrap());
                }
            }
        }

        stack.pop().unwrap()
    }
}
/* Submission Code Ends */

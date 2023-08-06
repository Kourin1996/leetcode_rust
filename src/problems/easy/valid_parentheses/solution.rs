pub struct Solution {}

/* Submission Code Begins */
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());

        for c in s.chars() {
            match c {
                '(' | '{' | '[' => {
                    stack.push(c);
                }
                ')' | '}' | ']' => match stack.pop() {
                    Some(t) => {
                        if c == ')' && t != '(' || c == '}' && t != '{' || c == ']' && t != '[' {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                },
                _ => {
                    return false;
                }
            }
        }

        stack.len() == 0
    }
}
/* Submission Code Ends */

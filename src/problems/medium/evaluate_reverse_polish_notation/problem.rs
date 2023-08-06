/*
Description:
You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.
Evaluate the expression. Return an integer that represents the value of the expression.

Note that:
The valid operators are '+', '-', '*', and '/'.
Each operand may be an integer or another expression.
The division between two integers always truncates toward zero.
There will not be any division by zero.
The input represents a valid arithmetic expression in a reverse polish notation.
The answer and all the intermediate calculations can be represented in a 32-bit integer.


Example 1:
Input: tokens = ["2","1","+","3","*"]
Output: 9

Example 2:
Input: tokens = ["4","13","5","/","+"]
Output: 6

Example 3:
Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
Output: 22

Constraints:
1 <= tokens.length <= 104
tokens[i] is either an operator: "+", "-", "*", or "/", or an integer in the range [-200, 200].
*/

#[cfg(test)]
mod tests {
    use crate::problems::medium::evaluate_reverse_polish_notation::solution::Solution;

    fn strs_to_strings(ss: Vec<&str>) -> Vec<String> {
        ss.into_iter().map(|s| String::from(s)).collect()
    }

    /* Test Cases */
    #[test]
    fn example1() {
        assert_eq!(
            Solution::eval_rpn(strs_to_strings(vec!["2", "1", "+", "3", "*"])),
            9,
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::eval_rpn(strs_to_strings(vec!["4", "13", "5", "/", "+"])),
            6,
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::eval_rpn(strs_to_strings(vec![
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ])),
            22,
        )
    }
}

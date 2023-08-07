/*
Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

Implement the MinStack class:

MinStack() initializes the stack object.
void push(int val) pushes the element val onto the stack.
void pop() removes the element on the top of the stack.
int top() gets the top element of the stack.
int getMin() retrieves the minimum element in the stack.
You must implement a solution with O(1) time complexity for each function.

Example 1:
Input
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]
Output
[null,null,null,null,-3,null,0,-2]

Constraints:
-231 <= val <= 231 - 1
Methods pop, top and getMin operations will always be called on non-empty stacks.
At most 3 * 104 calls will be made to push, pop, top, and getMin.
*/

#[cfg(test)]
mod tests {
    use crate::problems::medium::min_stack::solution::MinStack;

    /* Test Cases */
    #[test]
    fn example1() {
        let mut min_stack = MinStack::new();

        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);

        assert_eq!(
            min_stack.get_min(),
            -3,
        );

        min_stack.pop();

        assert_eq!(
            min_stack.top(),
            0
        );

        assert_eq!(
            min_stack.get_min(),
            -2,
        );
    }
}

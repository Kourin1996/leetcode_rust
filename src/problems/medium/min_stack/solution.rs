/* Submission Code Begins */
pub struct MinStack {
    stack: Vec<(i32, i32)>
}

impl MinStack {
    pub fn new() -> Self {
        MinStack { stack: vec![] }
    }
    
    pub fn push(&mut self, val: i32) {
        match self.stack.last() {
            Some((top, min)) => {
                let new_min = if *min > val { val } else { *min };

                self.stack.push((val, new_min));
            },
            None => {
                self.stack.push((val, val));
            }
        }      
    }
    
    pub fn pop(&mut self) {
        self.stack.pop();
    }
    
    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }
    
    pub fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}
/* Submission Code Ends */

// Create queue with two stacks


use crate::stack::Stack;

struct Queue<T> {
    in_stack: Stack<T>,
    out_stack: Stack<T>,
}

impl<T:Copy> Queue<T> {
    fn new() -> Queue<T> {
        Queue {
            in_stack: Stack::new(),
            out_stack: Stack::new(),
        }
    }
    fn add(&mut self,element: T) {
        self.in_stack.push(element);
    }

    fn drain(&mut self) {
        while let Some(el) = self.in_stack.pop() {
            self.out_stack.push(el);
        }
    }

    fn back_drain(&mut self) {
        while let Some(el) = self.out_stack.pop() {
            self.in_stack.push(el);
        }
    }

    fn remove(&mut self) -> Option<T> {
        self.drain();
        let out =self.out_stack.pop();
        self.back_drain();
        out
    }

    fn peek(&mut self) -> Option<T> {
        self.drain();
        let out = self.out_stack.peek().copied();
        self.back_drain();
        out
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qstack() {
        let mut q = Queue::new();
        q.add(1);
        q.add(2);
        let el = q.remove().expect("get element '1'");
        assert_eq!(el,1);
        let el = q.peek().expect("get element '2'");
        assert_eq!(el, 2);
        let el = q.peek().expect("get element '2'");
        assert_eq!(el, 2);
    }
}
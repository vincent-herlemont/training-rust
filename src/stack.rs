// Stack FILO (First-Int-Last-Out)


pub struct Stack<T> {
    v: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {v:vec![]}
    }

    pub fn push(&mut self,element: T) {
        self.v.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.v.pop()
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.v.last()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        assert_eq!(s.v, vec![1, 2]);
        let el = s.pop().expect("get element '2'");
        assert_eq!(el,2);
        assert_eq!(s.v, vec![1]);
        let el = s.peek().expect("get element '1'");
        assert_eq!(el,&1);
        assert_eq!(s.v, vec![1]);
    }
}
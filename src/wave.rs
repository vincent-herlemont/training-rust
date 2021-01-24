#[derive(Debug)]
struct Queue<T> {
    v: Vec<T>
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue {v: vec![]}
    }

    fn add(&mut self,element: T) {
        self.v.splice(..0,vec![element]);
    }

    fn remove(&mut self) -> Option<T> {
        self.v.pop()
    }
}

fn wave<T>(q1:&mut Queue<T>,q2:&mut Queue<T>) -> Queue<T> {
    let mut q = Queue::new();
    while !q1.v.is_empty() || !q2.v.is_empty() {
        if let Some(i) = q1.remove() {
            q.add(i);
        }
        if let Some(i) = q2.remove() {
            q.add(i);
        }
    }
    q
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q =  Queue::<_>::new();
        q.add(3);
        q.add(4);
        q.add(5);
        assert_eq!(q.v,vec![5,4,3]);
        q.remove();
        assert_eq!(q.v,vec![5,4]);
    }

    #[test]
    fn test_wave() {
        let mut q1 =  Queue::<_>::new();
        q1.add(1);
        q1.add(2);
        q1.add(3);
        let mut q2 =  Queue::<_>::new();
        q2.add(4);
        q2.add(5);
        q2.add(6);
        q2.add(7);
        q2.add(8);
        let q = wave(&mut q1, &mut q2);
        assert_eq!(q1.v,vec![]);
        assert_eq!(q2.v,vec![]);
        assert_eq!(q.v,vec![8,7,6,3,5,2,4,1]);
    }
}
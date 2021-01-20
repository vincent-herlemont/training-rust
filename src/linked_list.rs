use std::option::Option::Some;
use std::cell::RefCell;
use std::rc::Rc;
use std::convert::{TryFrom, TryInto};

type RefNode<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<RefNode<T>>
}

impl<T> Node<T> {
    fn new(data:T, node: Option<Node<T>>) -> Node<T> {
        Node {
            data,
            next: node.map(|node| Rc::new(RefCell::new(node)))
        }
    }
}

impl<T> TryFrom<RefNode<T>> for Node<T> {
    type Error = &'static str;

    fn try_from(node: RefNode<T>) -> Result<Self, Self::Error> {
        if let Ok(node) = Rc::try_unwrap(node) {
            Ok(node.into_inner())
        } else {
            Err("can not retrieve Node from RefNode")
        }
    }
}

struct NodeItr<T> {
    current: Option<RefNode<T>>
}

impl<T> Iterator for NodeItr<T> {
    type Item = RefNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.take() {
            if let Some(next) = &current.borrow().next {
                self.current = Option::from(Rc::clone(next));
            }
            Some(Rc::clone(&current))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<RefNode<T>>
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None
        }
    }

    fn iter(&self) -> NodeItr<T> {
        NodeItr {
            current: self.head.as_ref().map(|node| Rc::clone(&node))
        }
    }

    fn insert_first(&mut self,data: T) {
        if let Some(next) = self.head.take() {
            let next: Node<_> = next.try_into().unwrap();
            let node = Node::new(data, Some(next));
            self.head = Some(Rc::new(RefCell::new(node)));
        } else {
            let node = Node::new(data,None);
            self.head = Some(Rc::new(RefCell::new(node)));
        }
    }

    fn size(&self) -> usize {
        self.iter().count()
    }

    fn get_first(&self) -> Option<RefNode<T>> {
        self.iter().next()
    }

    fn get_last(&self) -> Option<RefNode<T>> {
        self.iter().last()
    }

    fn remove_first(&mut self) {
        if self.size() == 1 {
            self.head = None;
        }
        let mut iter = self.iter().skip(1);
        if let Some(node) = iter.next() {
            self.head = Some(Rc::clone(&node))
        }
    }

    fn remove_last(&mut self) {
        let size = self.size();
        if size >= 2 {
            let mut iter = self.iter();
            let before_last_node = iter.nth(size-2).unwrap();
            let mut node = before_last_node.borrow_mut();
            node.next = None;
        } else {
            self.head = None;
        }
    }

    fn insert_last(&mut self, data: T) {
        if let Some(last) = self.get_last() {
            let mut last = last.borrow_mut();
            let new_node = Rc::new(RefCell::new(Node::new(data,None)));
            last.next = Some(new_node);
        }
    }

    fn get_at(&self,n: usize) -> Option<RefNode<T>> {
        let mut iter = self.iter();
        iter.nth(n).map(|node| Rc::clone(&node))
    }

    fn remove_at(&mut self,n: usize) {
        let size = self.size();

        match n {
            0 => self.remove_first(),
            n if n < size => {
                let before_target = self.get_at(n-1).unwrap();
                let mut before_target = before_target.borrow_mut();
                let target = before_target.next.as_ref().map(|ref_node| Rc::clone(ref_node)).unwrap();
                let after_target = target.borrow_mut().next.take();
                before_target.next = after_target
            },
            n if n == size => self.remove_last(),
            _ => ()
        }
    }

    fn clear(&mut self) {
        self.head = None
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::option::Option::Some;

    #[test]
    fn test_remove_at() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_first(3);
        l.remove_at(1);
        assert_eq!(l.get_first().unwrap().borrow().data, 3);
        assert_eq!(l.get_last().unwrap().borrow().data, 1);
        assert_eq!(l.size(),2);
        l.remove_at(1);
        assert_eq!(l.size(),1);
        assert_eq!(l.get_first().unwrap().borrow().data, 3);
        assert_eq!(l.get_last().unwrap().borrow().data, 3);
        l.remove_at(0);
        assert_eq!(l.size(),0);
    }

    #[test]
    fn test_get_at() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_first(3);
        assert_eq!(l.get_at(0).unwrap().borrow().data, 3);
        assert_eq!(l.get_at(1).unwrap().borrow().data, 2);
        assert_eq!(l.get_at(2).unwrap().borrow().data, 1);
    }

    #[test]
    fn test_insert_last() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_last(0);
        assert_eq!(l.get_last().unwrap().borrow().data, 0);
    }

    #[test]
    fn test_remove_last() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_first(3);
        l.remove_last();
        assert_eq!(l.get_last().unwrap().borrow().data,2);
        assert_eq!(l.size(),2);
        l.remove_last();
        assert_eq!(l.size(),1);
        l.remove_last();
        assert_eq!(l.size(),0);
        l.remove_last();
        assert_eq!(l.size(),0);
    }

    #[test]
    fn test_remove_first() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_first(3);
        l.remove_first();
        assert_eq!(l.get_first().unwrap().borrow().data,2);
        assert_eq!(l.size(),2);
    }

    #[test]
    fn test_size() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_first(3);
        let c = l.size();
        assert_eq!(c,3);
    }

    #[test]
    fn test_get_first() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_first(3);
        let n = l.get_first().unwrap();
        assert_eq!(n.borrow().data,3);
    }

    #[test]
    fn test_get_last() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_first(3);
        let n = l.get_last().unwrap();
        assert_eq!(n.borrow().data,1);
    }

    #[test]
    fn test_clear() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.clear();
        assert_eq!(l.size(),0);
    }

    #[test]
    fn test_node() {
        let n = Node::new(1,None);
        let _ = Node::new(2, Some(n));
    }
}
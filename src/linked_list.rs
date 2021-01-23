use std::option::Option::Some;
use std::cell::{RefCell};
use std::rc::Rc;
use std::iter::FromIterator;

type RefNode<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug, Eq, PartialEq)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<RefNode<T>>
}

impl<T> Node<T> {
    pub fn new_ref_node(data:T) -> RefNode<T> {
        let node = Node {
            data,
            next:None
        };
        Rc::new(RefCell::new(node))
    }

    pub fn from_ref_node(data:T, ref_node: Option<RefNode<T>>) -> RefNode<T> {
        let node = Node {
            data,
            next: ref_node
        };
        Rc::new(RefCell::new(node))
    }
}

pub struct NodeItr<T> {
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

pub struct NodeIntoItr<T> {
    current: Option<RefNode<T>>
}

impl<T> Iterator for NodeIntoItr<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.take() {
            if let Some(next) = &current.borrow().next {
                self.current = Option::from(Rc::clone(next))
            }
            if let Ok(node) = Rc::try_unwrap(current) {
                Some(node.into_inner().data)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = NodeIntoItr<T>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIntoItr {
            current: self.head
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut l = LinkedList::new();
        for n in iter {
            l.insert_last(n);
        }
        l
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<RefNode<T>>
}

impl<T> From<RefNode<T>> for LinkedList<T> {
    fn from(node: RefNode<T>) -> Self {
        LinkedList {
            head: Some(node),
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None
        }
    }

    pub fn iter(&self) -> NodeItr<T> {
        NodeItr {
            current: self.head.as_ref().map(|node| Rc::clone(&node))
        }
    }

    pub fn insert_first(&mut self,data: T) {
        if let Some(next) = self.head.take() {
            self.head = Some(Node::from_ref_node(data, Some(next)));
        } else {
            self.head = Some(Node::new_ref_node(data));
        }
    }

    pub fn size(&self) -> usize {
        self.iter().count()
    }

    pub fn get_first(&self) -> Option<RefNode<T>> {
        self.iter().next()
    }

    pub fn get_last(&self) -> Option<RefNode<T>> {
        self.iter().last()
    }

    pub fn remove_first(&mut self) {
        if self.size() == 1 {
            self.head = None;
        }
        let mut iter = self.iter().skip(1);
        if let Some(node) = iter.next() {
            self.head = Some(Rc::clone(&node))
        }
    }

    pub fn remove_last(&mut self) {
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

    pub fn insert_last(&mut self, data: T) {
        if let Some(last) = self.get_last() {
            let mut last = last.borrow_mut();
            last.next = Some(Node::new_ref_node(data));
        } else {
            self.insert_first(data);
        }
    }

    pub fn get_at(&self,n: usize) -> Option<RefNode<T>> {
        let mut iter = self.iter();
        iter.nth(n).map(|node| Rc::clone(&node))
    }

    pub fn remove_at(&mut self,n: usize) {
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

    pub fn insert_at(&mut self, n: usize, data: T) {
        let size = self.size();
        match n {
            0 => self.insert_first(data),
            n if n < size => {
                let before_new = self.get_at(n-1).unwrap();
                let mut before_new = before_new.borrow_mut();
                before_new.next = Some(Node::from_ref_node(data,before_new.next.take()));
            },
            n if n == size => self.insert_last(data),
            _ => ()
        }
    }

    pub fn clear(&mut self) {
        self.head = None
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::option::Option::Some;


    #[test]
    fn test_insert_at() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_first(3);
        l.insert_at(1,99);
        assert_eq!(l.get_at(1).unwrap().borrow().data, 99);
        assert_eq!(l.size(),4);
        l.insert_at(0,88);
        assert_eq!(l.get_first().unwrap().borrow().data, 88);
        assert_eq!(l.size(),5);
        l.insert_at(5,100);
        assert_eq!(l.get_last().unwrap().borrow().data, 100);
        assert_eq!(l.size(),6);
    }

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
        let n = Node::new_ref_node(1);
        let _ = Node::from_ref_node(2, Some(n));

    }

    #[test]
    fn test_into_iter() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_first(3);
        let v: Vec<_> = l.into_iter().collect();
        assert_eq!(v,vec![3,2,1]);
    }

    #[test]
    fn test_from_iter() {
        let v = vec![3,2,1];
        let l: LinkedList<_> = v.into_iter().collect();
        assert_eq!(l.size(),3);
        assert_eq!(l.get_first().unwrap().borrow().data,3);
        assert_eq!(l.get_last().unwrap().borrow().data,1);
    }
}
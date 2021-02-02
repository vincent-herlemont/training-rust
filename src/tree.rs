use std::fmt::Debug;
use std::cell::RefCell;
use std::rc::Rc;

type RefNode<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data:T,
    children:Vec<RefNode<T>>
}

impl<T> Node<T> {
    fn new(data:T) -> RefNode<T> {
        let node = Node {
            data,
            children:vec![],
        };
        Rc::new(RefCell::new(node))
    }

    fn push_children(&mut self,node: RefNode<T>) {
        self.children.push(node)
    }
}

#[derive(Debug)]
struct Tree<T> {
    head: Option<RefNode<T>>
}

impl<T> From<RefNode<T>> for Tree<T> {
    fn from(ref_node: RefNode<T>) -> Self {
        Tree {
            head: Some(ref_node)
        }
    }
}

impl<T> Tree<T> {
    fn depth_first_iter(&self) -> DepthFirstIter<T> {
        DepthFirstIter {
            current: self.head.as_ref().map(|node| Rc::clone(node))
        }
    }
}

struct DepthFirstIter<T> {
    current: Option<RefNode<T>>
}

impl<T> Iterator for DepthFirstIter<T> {
    type Item = RefNode<T>;

    fn next(&mut self) -> Option<Self::Item> {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree(){
        let n1 = Node::new(1);
        let n2 = Node::new(2);
        n1.borrow_mut().push_children(n2);
        let t: Tree<_> = n1.into();
        dbg!(&t);
        let mut i = t.depth_first_iter();
        {
            let n = i.next();
            dbg!(n);
        }
    }
}
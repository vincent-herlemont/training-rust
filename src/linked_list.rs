use std::option::Option::Some;
use std::iter::FromIterator;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Box<Option<Node<T>>>
}

impl<T> Node<T> {
    fn new(data:T,ref_node: Option<Node<T>>) -> Node<T> {
        Node {
            data,
            next: Box::new(ref_node)
        }
    }

    fn iter(&self) -> NodeItr<'_,T> {
        NodeItr {
            node: Some(self)
        }
    }
}

impl<T> IntoIterator for Node<T> {
    type Item = T;
    type IntoIter = NodeIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIntoIter {
            node: Some(self)
        }
    }
}

impl<A> FromIterator<A> for Node<A> {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        let v: Vec<_> = iter.into_iter().collect();
        let mut into_iter_v = v.into_iter().rev();
        let head = into_iter_v.next().expect("collection into Node must have at lest one element");
        let mut head = Node::new(head, None);
        for el in into_iter_v {
            head = Node::new(el,Some(head));
        }
        head
    }
}

struct NodeIntoIter<T> {
    node: Option<Node<T>>
}

impl<T> Iterator for NodeIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut node) = self.node.take() {
            self.node = node.next.take();
            Some(node.data)
        } else {
            None
        }
    }
}

struct NodeItr<'n,T> {
    node: Option<&'n Node<T>>
}

impl<'n,T> Iterator for NodeItr<'n,T> {
    type Item = &'n Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node {
            let out = self.node;
            self.node = (&*node.next).into();
            // self.node = node.next.as_ref().into(); // &*... for Option data is equivalent to as_ref() method.
            out
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Node<T>>
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None
        }
    }

    fn insert_first(&mut self,data: T) {
        self.head = Some(Node::new(data,self.head.take()));
    }

    fn size(&self) -> usize {
        if let Some(node) = &self.head {
            node.iter().count()
        } else {
            0
        }
    }

    fn get_first(&self) -> Option<&Node<T>> {
        self.head.as_ref()
    }

    fn get_last(&self) -> Option<&Node<T>> {
        self.head.as_ref().and_then(|node| {
            node.iter().last()
        })
    }

    fn remove_first(&mut self) {
        self.head = self.head.as_mut().and_then(|node| {
            node.next.take()
        });
    }

    fn remove_last(&mut self) {
        if let Some(node) = self.head.take() {
            let mut v: Vec<_> = node.into_iter().collect();
            v.pop();
            if v.is_empty() {
                self.head = None;
            } else {
                self.head = Some(v.into_iter().collect())
            }
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
    fn test_remove_last() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_first(3);
        l.remove_last();
        assert_eq!(l.get_last().unwrap().data,2);
        assert_eq!(l.size(),2);
        l.remove_last();
        l.remove_last();
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
        assert_eq!(l.get_first().unwrap().data,2);
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
        assert_eq!(n.data,3);
    }

    #[test]
    fn test_get_last() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_first(3);
        let n = l.get_last().unwrap();
        assert_eq!(n.data,1);
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
        let n = Node::new(2, Some(n));
        let mut iter = n.iter();
        assert_eq!(iter.next().unwrap().data,2);
        assert_eq!(iter.next().unwrap().data,1);
        assert!(iter.next().is_none());

        let v = vec![1,2,3];
        let n: Node<_> = v.into_iter().collect();
        let v: Vec<_> = n.into_iter().collect();
        assert_eq!(v, vec![1,2,3]);
    }
}
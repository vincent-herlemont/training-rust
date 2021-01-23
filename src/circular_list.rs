use crate::linked_list::LinkedList;

fn is_circular_list<T: Eq + PartialEq>(list: &LinkedList<T>) -> bool {
    for (index,slow_node) in list.iter().enumerate() {
        for fast_node in list.iter().skip(index+1) {
            if *fast_node.borrow() == *slow_node.borrow() {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked_list::{LinkedList, Node};
    use std::rc::Rc;

    #[test]
    fn test_is_not_circular_list() {
        let ref_node_1 = Node::new_ref_node(1);
        let ref_node_2 = Node::from_ref_node(2, Some(Rc::clone(&ref_node_1)));
        let ref_node_3 = Node::from_ref_node(3,Some(ref_node_2));
        let cl : LinkedList<_> = ref_node_3.into();
        assert!(!is_circular_list(&cl));
    }

    #[test]
    fn test_is_circular_list() {
        let ref_node_1 = Node::new_ref_node(1);
        let ref_node_2 = Node::from_ref_node(2, Some(Rc::clone(&ref_node_1)));
        let ref_node_3 = Node::from_ref_node(3,Some(ref_node_2));
        ref_node_1.borrow_mut().next = Some(ref_node_3);
        let cl : LinkedList<_> = ref_node_1.into();
        assert!(is_circular_list(&cl));
    }
}
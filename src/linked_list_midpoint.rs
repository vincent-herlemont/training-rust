use crate::linked_list::LinkedList;

// Return the value present at the middle of the list
fn midpoint<T:Copy>(list: LinkedList<T>) -> Option<T> {
    let mut slow_iter = list.iter();
    let mut fast_iter = list.iter().peekable();
    let mut out= None;
    while fast_iter.peek().is_some() {

        // fast_iter advance twice more faster to slow_iter.
        // in order to when fast_iter has reach the end of the list
        // slow_iter is at the middle of the list.
        fast_iter.next();
        fast_iter.next();
        out = slow_iter.next()
    }

    out.map(|node|node.borrow().data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midpoint() {
        let l: LinkedList<_> = vec![1,2,3,4,5].into_iter().collect();
        assert_eq!(midpoint(l).unwrap(),3);
        let l: LinkedList<_> = vec![1,2,3,4].into_iter().collect();
        assert_eq!(midpoint(l).unwrap(),2);
        let l: LinkedList<_> = vec![1].into_iter().collect();
        assert_eq!(midpoint(l).unwrap(),1);
    }

    #[test]
    fn test_midpoint_empty_list() {
        let l:LinkedList<i32> = LinkedList::new();
        assert_eq!(midpoint(l),None);
    }
}

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    // fn values(&self) -> Box<dyn Iterator<Item=i32>> {
    //     let v_data = vec![self.data];
    //     let iter = match self.next.as_ref() {
    //         Some(next) =>  v_data.into_iter().chain(next.values()),
    //         None => v_data.into_iter()
    //     };
    //     Box::new(iter)
    // }
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Node>
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
        }
    }

    fn insert_first(&mut self,data: i32) {
        self.head = self.head.take().map(|last| {
            Node {
                data,
                next: Some(Box::new(last)),
            }
        }).or(Some(Node {
            data,
            next: None,
        }))
    }

    // fn iter(&self) -> Box<dyn Iterator<Item=i32>> {
    //     if let Some(head) = self.head.as_ref() {
    //         println!("T1");
    //         head.values()
    //     } else {
    //         Box::new(vec![].into_iter())
    //     }
    // }
    //
    // fn size(&self) -> usize {
    //     if let Some(head) = self.head.as_ref() {
    //         head.values().collect::<Vec<_>>().len()
    //     } else {
    //         0
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let mut l = LinkedList::new();
        l.insert_first(1);
        l.insert_first(2);
        l.insert_first(3);
        // println!("{:?}", l);
        // let mut iter = l.iter();
        // let v = iter.next();
        // println!("{:?}",v);
        // let c = l.size();
        // assert_eq!(c,3);
    }
}
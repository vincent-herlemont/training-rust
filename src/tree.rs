use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    data:i32,
    children:Vec<Node>
}

impl Node {
    fn new(data:i32) -> Node {
        let node = Node {
            data,
            children:vec![],
        };
        node
    }

    fn push_children(&mut self,node: Node) {
        self.children.push(node)
    }

    fn values<'a>(&'a self) -> Box<dyn Iterator<Item=i32> + 'a> {
        let v = vec![self.data.clone()];
        Box::new(
            v.into_iter().chain(self.children.iter().map(|node| {
                node.values()
            }).flatten())
        )
    }
}

#[derive(Debug)]
struct Tree {
    head: Option<Node>
}

impl From<Node> for Tree {
    fn from(node: Node) -> Self {
        Tree {
            head: Some(node)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree(){
        let mut n1 = Node::new(1);
        let n2 = Node::new(2);
        let mut n3 = Node::new(3);
        let n31 = Node::new(31);
        {
            n3.push_children(n31);
        }
        {
            n1.push_children(n2);
            n1.push_children(n3);

            let values: Vec<_> = n1.values().collect();
            println!("{:?}", values);
        }





        // let t: Tree<_> = n1.into();

        // println!("{}", n1.as_ptr() == n2.as_ptr());
        // println!("{}", n2.as_ptr() == n3.as_ptr());

        // dbg!(&t);
        // let mut i = t.depth_first_iter();
        // {
        //     let n = i.next();
        //     dbg!(n);
        // }
    }
}
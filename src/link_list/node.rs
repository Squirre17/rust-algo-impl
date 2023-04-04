use std::cell::RefCell;
use std::rc::Rc;
pub type NodeRef = Rc<RefCell<Node>>;
pub type NodeOpt = Option<NodeRef>;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub data : i32,
    pub next : NodeOpt
}

impl Node {
    pub fn new(data : i32) -> NodeRef {
        Rc::new(RefCell::new(Node { 
            data, 
            next : None 
        }))
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("drop with data {}", self.data);
    }
}

pub struct LinkListIter {
    cur : NodeOpt
}
impl LinkListIter {
    pub fn new(start : NodeOpt) -> Self {
        LinkListIter { 
            cur : start 
        }
    }
}
impl Iterator for LinkListIter {

    type Item = NodeRef;
    fn next(&mut self) -> NodeOpt {
        let mut res = None;
        
        self.cur = match self.cur.as_ref() {

            Some(cur) => {
                res = Some(Rc::clone(cur));
                match cur.borrow_mut().next {
                    Some(ref next) => {
                        Some(Rc::clone(next))
                    },
                    None => {
                        None
                    }
                }
            },
            None => {
                None
            }
        };

        res
    }
}

mod test {
    use super::*;
    #[test]
    fn test1() {
        let node : NodeRef = Node::new(1);
        let mut lli = LinkListIter::new(
            Some(node)
        );
        // let node : &Node = lli.next().take().unwrap().borrow();
        // assert_eq!(node.borrow().data, 1);
        assert_eq!(lli.next().unwrap().borrow_mut().data, 1);
    }
}
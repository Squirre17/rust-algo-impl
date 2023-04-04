mod node;

use std::ops::Deref;
use std::rc::Rc;

use crate::link_list::node::{Node, NodeOpt, NodeRef};

#[derive(Debug)]
struct LinkedList{
    head       : NodeOpt,
    tail       : NodeOpt,

    pub length : usize,  
}

impl LinkedList {
    pub fn new(data : i32) -> Self{
        let node = Node::new(data);
        LinkedList {
            head   : Some(Rc::clone(&node)),
            tail   : Some(Rc::clone(&node)),
            length : 0
        }
    }
    pub fn push_end(&mut self, elem : i32) {
        let node : NodeRef = Node::new(elem);
        
        match &self.head {
            Some(_) => {
                match &self.tail {
                    Some(node_ref) => {
                        node_ref.borrow_mut().next = Some(Rc::clone(&node));
                        self.tail = Some(Rc::clone(&node));
                    },
                    None => {
                        unreachable!()
                    }
                }
            },
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
        }
    }
    pub fn push_front(&mut self, elem : i32){

        let node : NodeRef = Node::new(elem);

        match &self.head.as_ref() {
            Some(head) => {
                node.borrow_mut().next = Some(Rc::clone(head));
                let new_head = Some(Rc::clone(&node));
                self.head = new_head;
            },
            None => {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(Rc::clone(&node));
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<i32> {

        let new_head;
        let ret;
        
        match &mut self.head {
            Some(node_ref) => {
                // node_ref : &Rc<RefCell<Node>>
                // print_type(node_ref);
                let mut node = node_ref.borrow_mut();
                
                if node.next.is_none() {
                    new_head = None;
                }
                else {
                    new_head = Some(Rc::clone(&node.next.as_mut().unwrap()));   
                }
                ret = node.data;

            },
            None => {
                return None;
            }
        }
        self.head = new_head;
        return Some(ret);
    }
    pub fn traverse(&self) {
        
        println!("--------------------- traverse ---------------------");
        let mut node_opt = self.head.clone();

        while let Some(node_ref) = node_opt {
            let node = node_ref.deref().borrow();
            println!("{}", node.data);

            // 不夺取Some的所有权而获得Some中元素访问的办法 => as_ref
            if let Some(next) = node.next.as_ref() {

                node_opt = Some(Rc::clone(next));

            }else {
                node_opt = None;
            }
        }
    }
    fn gen_arr(&self) -> Vec<i32> {

        let mut node_opt = self.head.clone();
        let mut arr = vec![];

        while let Some(node_ref) = node_opt {

            let node = node_ref.deref().borrow();
            arr.push(node.data.clone());

            if let Some(next) = node.next.as_ref() {

                node_opt = Some(Rc::clone(next));

            }else {
                node_opt = None;
            }
        }
        arr
    }
}

mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut ll = LinkedList::new(1);
        ll.push_end(2);
        ll.push_end(3);
        ll.push_end(3);
        ll.push_front(1);
        assert_eq!(ll.gen_arr(), vec![1,1,2,3,3]);
        ll.pop_front();
        ll.pop_front();
        ll.pop_front();
        assert_eq!(ll.gen_arr(), vec![3,3]);
    }
}
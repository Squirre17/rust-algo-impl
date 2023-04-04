use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;
use std::fmt::Display;

type TnRef  = Rc<RefCell<TreeNode>>;
type TnWeak = Weak<RefCell<TreeNode>>;
// type TnOpt  = Option<TnRef>;

#[derive(Debug)]
struct TreeNode {
    pub is_leaf  : bool,
    pub value    : u32,
    pub children : Vec<TnRef>,
    pub parent   : Option<TnWeak>,
}

impl TreeNode {
    pub fn new(value : Option<u32>) -> Self {
        // pass concrete value repr that is a leaf node
        TreeNode {
            is_leaf  : if value.is_none() {false} else {true},
            value    : value.unwrap_or(0),
            children : Vec::new(),
            parent   : None
        }
    }
    pub fn add_child(&mut self, node : &TnRef) {
        self.children.push(
            Rc::clone(node)
        )
    }
    pub fn pprint(&self) -> String {
        if self.is_leaf {
            return format!("{}", self.value);
        }
        String::from("[") + &self.children
                                 .iter()
                                 .map(|tn| tn.deref().borrow().pprint())
                                 .collect::<Vec<String>>()
                                 .join(",")
                                 + "]"
    }

}
impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pprint())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(None);
        root.add_child(&Rc::new(RefCell::new(
            TreeNode::new(Some(1))
        )));
        root.add_child(&Rc::new(RefCell::new(
            TreeNode::new(Some(2))
        )));
        assert_eq!(root.pprint(), String::from("[1,2]"));
    }
}


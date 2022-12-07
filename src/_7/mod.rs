use std::{cell::RefCell, rc::Rc};

struct Node<T> {
    pub value: Option<T>,
    pub children: Option<Vec<Box<Node<T>>>>,
    pub parent: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: Option<T>) -> Self {
        return Self {
            value,
            children: None,
            parent: None,
        };
    }
    pub fn add_child(&mut self, new_node: Box<Node<T>>) {
        match &mut self.children {
            Some(children) => {
                children.push(new_node);
            }
            None => {
                self.children = Some(vec![new_node]);
            }
        }
    }
}

pub fn get_7_first() -> i32 {
    let content = include_str!("../../inputs/7/input.data");
    2
}
pub fn get_7_second() -> i32 {
    let content = include_str!("../../inputs/7/input.data");
    2
}

#[cfg(test)]
mod test_7 {
    use super::*;

    #[test]
    fn test_new_node() {
        let single_node = Node::new(Some(0));
        assert_eq!(single_node.value, Some(0));
        assert!(single_node.children.is_none());
    }

    #[test]
    fn test_add_child_is_appending_root() {
        let mut single_node = Node::new(Some(0));
        let child_node = Node::new(Some(1));
        single_node.add_child(Box::new(child_node));
        assert!(single_node.children.is_some());
        let children = single_node.children;
        match children {
            Some(children) => {
                assert!(children.get(0).is_some());
                assert_eq!(children.get(0).unwrap().value, Some(1));
            }
            None => panic!("Should be exactly one child"),
        }
    }
}

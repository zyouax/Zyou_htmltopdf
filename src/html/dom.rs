use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Rc<RefCell<Node>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Element(String),
    Text(String),
    Comment(String),
}

impl Node {
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
    }
}
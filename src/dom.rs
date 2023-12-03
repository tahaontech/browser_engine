use std::collections::{HashMap, HashSet};

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    // data common to all nodes:
    pub children: Vec<Node>,

    // data specific to each node type:
    pub node_type: NodeType
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new()
        }
    }
}

// constructor of text node
pub fn text(data: String) -> Node {
    Node { children: vec![], node_type: NodeType::Text(data) }
}

// constructor of element node
pub fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node { 
        children: children, 
        node_type: NodeType::Element(ElementData { 
            tag_name: name, 
            attributes: attrs 
        }) 
    }
}
//! Handles all process related matters

use crate::player::*;

#[derive(Debug, Clone)]
pub struct ProcessNode {
    name: String,
    description: String,
    display_text: String,
}

impl ProcessNode {
    pub fn new<'a>(name: &'a str, description: &'a str, display_text: &'a str) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            display_text: display_text.to_owned(),
        }
    }

}

#[derive(Debug, Clone)]
pub struct Process {
    process: Vec<ProcessNode>,
    current_node: Option<ProcessNode>,
}

impl Process {

    pub fn new() -> Self {
        Process { process: vec![], current_node: None }
    }

    pub fn add_node<'a>(&'a mut self, name: &'a str, description: &'a str, display_text: &'a str) -> &mut Self {
        let mut new_node = ProcessNode::new(name, description, display_text);
        self.process.push(new_node);
        self
    }

}





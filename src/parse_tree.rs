#![allow(dead_code)]

use crate::token::Token;

pub enum NodeType {
    FunctionDefinition,
    AssignmentStatement,
    // Add other node types here
}

pub struct ParseTree {
    token: Token,
    node_type: NodeType,
    children: Vec<Box<ParseTree>>,
}

impl ParseTree {
    pub fn new(token: Token, node_type: NodeType) -> ParseTree {
        ParseTree {
            token,
            node_type,
            children: vec![],
        }
    }

    pub fn push(&mut self, tree: ParseTree) {
        self.children.push(Box::new(tree));
    }

    pub fn node_string(&self) -> String {
        format!("{:?}", self.token)
    }

    fn print_recursively(&self, level: usize) {
        let shift = 2 * level;
        print!("{:1$}", "", shift);
        println!("{}", self.node_string());
        for child in &self.children {
            child.as_ref().print_recursively(level + 1);
        }
    }

    pub fn print(&self) {
        self.print_recursively(0);
    }
}

use crate::dom;
use std::collections::HashMap;

/// Parse an HTML document and return the root element.
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser { pos: 0, input: source }.parse_nodes();

    // If the document contains a root element, just return it. Otherwise, create one.
    if nodes.len() == 1 {
        nodes.remove(0)
    } else {
        dom::elem("html".to_string(), HashMap::new(), nodes)
    }
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    /// Parse a sequence of sibling nodes.
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = vec!();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    /// Parse a single node.
    fn parse_node(&mut self) -> dom::Node {
        if self.starts_with("<") {
            self.parse_element()
        } else {
            self.parse_text()
        }
    }
use crate::dom::{Node, NodeType, ElementData};
use crate::css::{Stylesheet, Rule, Selector, SimpleSelector, Value, Specificity};
use std::collections::HashMap;

/// Map from CSS property names to values.
pub type PropertyMap = HashMap<String, Value>;

/// A node with associated style data.
pub struct StyledNode<'a> {
    pub node: &'a Node,
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}
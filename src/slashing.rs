use crate::node::{Node, NodeState};

pub fn slash(node: &mut Node) {
    node.loyalty = 0.0;
    node.honesty = 0.0;
    node.state = NodeState::Slashed;
}

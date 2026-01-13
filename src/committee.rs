use crate::node::{Node, NodeState};
use crate::vrf::vrf_select;

pub fn select_committee(nodes: &Vec<Node>, size: usize) -> Vec<Node> {
    let old: Vec<Node> = nodes.iter()
        .filter(|n| matches!(n.state, NodeState::Old))
        .cloned()
        .collect();

    let new: Vec<Node> = nodes.iter()
        .filter(|n| matches!(n.state, NodeState::New))
        .cloned()
        .collect();

    let old_count = (size as f64 * 0.6) as usize;
    let new_count = size - old_count;

    let mut committee = Vec::new();

    for _ in 0..old_count {
        if !old.is_empty() {
            committee.push(vrf_select(&old));
        }
    }

    for _ in 0..new_count {
        if !new.is_empty() {
            committee.push(vrf_select(&new));
        }
    }

    committee
}

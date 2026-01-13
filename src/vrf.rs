use rand::{thread_rng, Rng};
use crate::node::{Node, NodeState};

/// Randomly selects a single node index
pub fn vrf_select(nodes: &Vec<Node>) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..nodes.len())
}

/// Selects a committee of node indices
pub fn select_committee(
    nodes: &Vec<Node>,
    old_ratio: f64,
    committee_size: usize,
) -> Vec<usize> {

    let old_nodes: Vec<_> = nodes.iter().enumerate()
        .filter(|(_, n)| n.loyalty == 1.0 && n.state != NodeState::Slashed)
        .map(|(i, _)| i)
        .collect();

    let new_nodes: Vec<_> = nodes.iter().enumerate()
        .filter(|(_, n)| n.loyalty < 1.0 && n.state != NodeState::Slashed)
        .map(|(i, _)| i)
        .collect();

    let old_count = ((committee_size as f64) * old_ratio).round() as usize;
    let new_count = committee_size - old_count;

    let mut rng = thread_rng();
    let mut committee = Vec::new();

    for _ in 0..old_count {
        if !old_nodes.is_empty() {
            let idx = rng.gen_range(0..old_nodes.len());
            committee.push(old_nodes[idx]);
        }
    }

    for _ in 0..new_count {
        if !new_nodes.is_empty() {
            let idx = rng.gen_range(0..new_nodes.len());
            committee.push(new_nodes[idx]);
        }
    }

    committee
}

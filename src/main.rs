mod node;
mod loyalty;
mod vrf;

use node::{Node, NodeState};
use loyalty::*;
use vrf::*;

fn main() {
    println!("LBHCM simulator booting...");

    let mut nodes: Vec<Node> = (0..50).map(|i| Node::new(i)).collect();
    let total_epochs = 10;
    let committee_size = 10;

    for epoch in 0..total_epochs {
        println!("Epoch {}", epoch + 1);

        // Increment age
        for node in nodes.iter_mut() {
            node.age_epochs += 1;
        }

        // Select proposer
        let proposer_idx = vrf_select(&nodes);
        println!("Proposer: Node {}", proposer_idx);

        // Select committee
        let committee_idxs = select_committee(&nodes, 0.6, committee_size);
        println!("Committee: {:?}", committee_idxs);

        // Simulate validation
        for &idx in &committee_idxs {
            let node = &mut nodes[idx];
            node.audits += 1;

            if rand::random::<f64>() < 0.1 {
                node.faults += 1;
            }

            node.honesty = honesty(node.faults, node.audits);
        }

        // Update loyalty
        let max_age = nodes.iter()
            .map(|n| age_metric(n.age_epochs))
            .fold(f64::NAN, f64::max);

        for node in nodes.iter_mut() {
            node.loyalty = loyalty(
                node.honesty,
                age_metric(node.age_epochs),
                max_age,
            );
        }

        // Slashing
        for node in nodes.iter_mut() {
            if node.honesty == 0.0 {
                node.loyalty = 0.0;
                node.state = NodeState::Slashed;
            }
        }

        let old_count = nodes.iter().filter(|n| n.loyalty == 1.0).count();
        let new_count = nodes.iter()
            .filter(|n| n.loyalty < 1.0 && n.state != NodeState::Slashed)
            .count();
        let slashed_count = nodes.iter()
            .filter(|n| n.state == NodeState::Slashed)
            .count();

        println!(
            "OLD pool: {}, NEW pool: {}, SLASHED: {}",
            old_count, new_count, slashed_count
        );
        println!("--------------------------------------------------");
    }

    println!("Simulation complete.");
}

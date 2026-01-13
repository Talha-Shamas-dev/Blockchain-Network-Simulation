#[derive(Clone, Debug, PartialEq)]
pub enum NodeState {
    New,
    Old,
    Slashed,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub id: u64,
    pub honesty: f64,
    pub age_epochs: u64,
    pub loyalty: f64,
    pub stake: u64,
    pub state: NodeState,
    pub faults: u64,
    pub audits: u64,
}

impl Node {
    pub fn new(id: u64) -> Self {
        Node {
            id,
            honesty: 1.0,
            age_epochs: 0,
            loyalty: 0.0,
            stake: 100,
            state: NodeState::New,
            faults: 0,
            audits: 0,
        }
    }
}

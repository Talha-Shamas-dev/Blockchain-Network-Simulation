#[derive(Clone)]
pub struct Block {
    pub proposer: u64,
    pub valid: bool,
    pub signatures: usize,
}

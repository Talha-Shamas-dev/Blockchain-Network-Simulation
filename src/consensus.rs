use crate::block::Block;

pub fn finalize(block: &Block, committee_size: usize) -> bool {
    block.signatures * 3 >= committee_size * 2
}

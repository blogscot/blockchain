use crate::block::{Block, MiningError};
use std::fmt;

crate struct Blockchain {
  blocks: Vec<Block>,
}

impl Blockchain {
  /// Initializes a new blockchain with a genesis block.
  pub fn genesis() -> Self {
    Self {
      blocks: vec![Block::genesis()],
    }
  }

  /// Adds a new block to the chain.
  pub fn add_block(&mut self, data: &str) -> Result<(), MiningError> {
    if let Some(prev_block) = self.blocks.last() {
      let new_block = Block::new(data, prev_block)?;
      self.blocks.push(new_block);
    }
    Ok(())
  }
}

impl fmt::Display for Blockchain {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for block in &self.blocks {
      writeln!(f, "{:?}", block).expect("Cannot print for some reason!");
    }
    writeln!(f)
  }
}

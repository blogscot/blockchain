use crate::block::{Block, MiningError};
use std::fmt;

crate struct Blockchain {
  blocks: Vec<Block>,
}

impl Blockchain {
  /// Initializes a new blockchain with a genesis block.
  pub fn new() -> Result<Self, MiningError> {
    Ok(Self {
      blocks: vec![Block::genesis()?],
    })
  }

  /// Adds a newly-mined block to the chain.
  /// Adding a block to an empty blockchain is an error: a genesis
  /// block needs to be created first.
  pub fn add_block(&mut self, data: &str) -> Result<(), MiningError> {
    let block = match self.blocks.last() {
      Some(prev_block) => Block::new(data, prev_block)?,
      None => return Err(MiningError::NoParent),
    };
    self.blocks.push(block);
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

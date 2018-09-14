use chrono::prelude::*;
use crypto::{digest::Digest, sha2::Sha256};
use num_bigint::BigUint;
use num_traits::One;
use std::{fmt, str, u64};

const DIFFICULTY: usize = 6;
const MAX_PROOF: u64 = u64::MAX;
const HASH_BYTE_SIZE: usize = 32;

type Sha256Hash = [u8; HASH_BYTE_SIZE];

#[derive(Clone)]
crate struct Block {
  // Headers
  timestamp: i64,
  prev_block_hash: Sha256Hash,
  proof: u64,

  // Body
  data: Vec<u8>,
}

impl Block {
  /// Creates a new block using the previous block's hash value.
  pub fn new(data: &str, prev_block: &Block) -> Result<Self, MiningError> {
    let (prev_block_hash, proof) = prev_block.hash()?;
    Ok(Self {
      timestamp: Utc::now().timestamp(),
      prev_block_hash,
      proof,
      data: data.into(),
    })
  }

  /// Creates a genesis block (i.e. a block with no parent)
  /// with the previous block's hash field set to all zeroes.
  pub fn genesis() -> Self {
    Self {
      timestamp: Utc::now().timestamp(),
      prev_block_hash: Sha256Hash::default(),
      proof: 0,
      data: "Genesis block".into(),
    }
  }

  /// Calculates the current blocks hash value.
  fn hash(&self) -> Result<(Sha256Hash, u64), MiningError> {
    let target = BigUint::one() << (256 - 4 * DIFFICULTY);

    for proof in 0..MAX_PROOF {
      let hash = self.calculate_hash(proof);
      let hash_int = BigUint::from_bytes_be(&hash);

      if hash_int < target {
        return Ok((hash, proof));
      }
    }
    Err(MiningError::IterationError)
  }

  /// Calculates the current block's hash value using a nonce value.
  fn calculate_hash(&self, nonce: u64) -> Sha256Hash {
    let mut vec = Vec::new();
    let mut hasher = Sha256::new();
    let mut hash = Sha256Hash::default();

    // Convert Block into byte vector
    vec.extend(&convert_u64_to_u8_array(self.timestamp as u64));
    vec.extend_from_slice(&self.prev_block_hash);
    vec.extend_from_slice(&convert_u64_to_u8_array(nonce));
    vec.extend(&self.data);

    hasher.input(&vec);
    hasher.result(&mut hash);
    hash
  }
}

/// Transforms a u64 integer into a little endian u8 array.
fn convert_u64_to_u8_array(val: u64) -> [u8; 8] {
  let mut output: [u8; 8] = [0; 8];
  for (index, _) in (0..8).enumerate() {
    output[index] = (val >> (index * 8)) as u8;
  }
  output
}

/// Converts a hash value into a string.
fn as_hex(hash: Sha256Hash) -> String {
  let mut output = String::new();
  for item in hash.iter() {
    output += &format!("{:x?}", item);
  }
  output
}

impl fmt::Debug for Block {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(
      f,
      " Timestamp: {}\n Prev Block Hash: {:?}\n Proof: {}\n Data: {:?}",
      self.timestamp,
      as_hex(self.prev_block_hash),
      self.proof,
      str::from_utf8(&self.data)
    )
  }
}

#[derive(Debug)]
crate enum MiningError {
  IterationError,
}

impl fmt::Display for MiningError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      MiningError::IterationError => write!(f, "Could not mine block, hit iteration limit"),
    }
  }
}

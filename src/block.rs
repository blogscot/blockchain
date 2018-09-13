use chrono::prelude::*;
use crypto::{digest::Digest, sha2::Sha256};
use num_bigint::BigUint;
use num_traits::One;
use std::{fmt, str};

const DIFFICULTY: usize = 6;
const MAX_NONCE: u64 = 100_000_000;
const HASH_BYTE_SIZE: usize = 32;

type Sha256Hash = [u8; HASH_BYTE_SIZE];

crate struct Block {
  // Headers
  timestamp: i64,
  prev_block_hash: Sha256Hash,
  pub block_hash: Sha256Hash,
  nonce: u64,

  // Body
  data: Vec<u8>,
}

impl Block {
  /// Creates a new block using the previous block's hash value.
  pub fn new(data: &str, prev_hash: Sha256Hash) -> Result<Self, MiningError> {
    let mut new_block = Self {
      timestamp: Utc::now().timestamp(),
      prev_block_hash: prev_hash,
      block_hash: Sha256Hash::default(),
      nonce: 0,
      data: data.into(),
    };

    new_block.set_hash()?;
    Ok(new_block)
  }

  /// Creates a genesis block (i.e. a block with no parent)
  /// with the previous block's hash field set to all zeroes.
  pub fn genesis() -> Result<Self, MiningError> {
    Self::new("Genesis block", Sha256Hash::default())
  }

  /// Sets the current blocks hash value.
  fn set_hash(&mut self) -> Result<(), MiningError> {
    let target = BigUint::one() << (256 - 4 * DIFFICULTY);

    for nonce in 0..MAX_NONCE {
      let hash = self.calculate_hash(nonce);
      let hash_int = BigUint::from_bytes_be(&hash);

      if hash_int < target {
        self.block_hash = hash;
        self.nonce = nonce;
        return Ok(());
      }
    }
    Err(MiningError::Iteration)
  }

  /// Calculates the current block's hash value using a nonce value.
  fn calculate_hash(&self, nonce: u64) -> Sha256Hash {
    let mut headers = self.headers();
    let mut hasher = Sha256::new();
    let mut hash = Sha256Hash::default();

    headers.extend_from_slice(&convert_u64_to_u8_array(nonce));
    hasher.input(&headers);
    hasher.result(&mut hash);
    hash
  }

  /// Collects the blocks header values (i.e. timestamp, and the
  /// previous blocks hash value) into a byte vector.
  fn headers(&self) -> Vec<u8> {
    let mut vec = Vec::new();

    vec.extend(&convert_u64_to_u8_array(self.timestamp as u64));
    vec.extend_from_slice(&self.prev_block_hash);
    vec
  }
}

/// Transforms a u64 integer into a little endian u8 array.
fn convert_u64_to_u8_array(val: u64) -> [u8; 8] {
  [
    val as u8,
    (val >> 8) as u8,
    (val >> 16) as u8,
    (val >> 24) as u8,
    (val >> 32) as u8,
    (val >> 40) as u8,
    (val >> 48) as u8,
    (val >> 56) as u8,
  ]
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
      " Timestap: {}\n Prev Block Hash: {:x?}\n Block Hash: {:x?}\n Nonce: {}\n Data: {:?}",
      self.timestamp,
      as_hex(self.prev_block_hash),
      as_hex(self.block_hash),
      self.nonce,
      str::from_utf8(&self.data)
    )
  }
}

#[derive(Debug)]
crate enum MiningError {
  Iteration,
  NoParent,
}

impl fmt::Display for MiningError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      MiningError::Iteration => write!(f, "Could not mine block, hit iteration limit"),
      MiningError::NoParent => write!(f, "Block has no parent"),
    }
  }
}

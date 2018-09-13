
## Intro

This repo is largely inspired by [asymmetric's blagh](https://asymmetric.github.io) blogpost [Implementing a Blockchain in Rust](https://asymmetric.github.io/2018/02/11/blockchain-rust/). I started hacking on the code snippets, eventually rewriting parts of the codebase. It's a good post, take a look.

### Running the code

This code was built and run using the nightly build, `1.29.0-nightly (af9e40c26 2018-07-05)`, along with the Rust `2018 Edition` options enabled.

Once you have cloned the repository to a local folder, type:

```zsh
➜  blockchain git:(master) ✗ cargo run --release
```

The code ran successfully with maximum values of:

```rust
const DIFFICULTY: usize = 6;
const MAX_NONCE: u64 = 100_000_000;
```

A `DIFFICULTY` value affects how many leading zero each hash value is required to have. For a value of 6 this would mean 3 leading zeros, as you can see in the output below. As may be evident already, more iterations are required to find a more _difficult_ hash value, hence, the more iterations the higher value of `MAX_NONCE` required. Using a release build, 100 million iterations doesn't take too long. 

### User Interface

```rust
fn run() -> Result<(), block::MiningError> {
  let mut chain = chain::Blockchain::new()?;
  chain.add_block("This is the first real block")?;
  chain.add_block("And this is the next one")?;
  println!("{}", chain);
  Ok(())
}
```

### Output

The output below shows an example blockchain containing a dummy starting block followed by two pieces of data stored in the chain. The initial `genesis` block is the special block which has the sole purpose of starting a new blockchain; when it is created it contains dummy data only.

For each subsequent block the previous block's hash value is stored in the current block (providing a encrypted continuous link between blocks, hence the name _chain_). The previous block's hash value is used to create the current blocks hash, along with other items such as the current `timestamp` and a `nonce`.

```
 Timestap: 1536840918
 Prev Block Hash: "00000000000000000000000000000000"
 Block Hash: "0008bd31e886a2fd335c2327a59e7d05e46cc43d8f7caae2ea78eee45849"
 Nonce: 823874
 Data: Ok("Genesis block")

 Timestap: 1536840918
 Prev Block Hash: "0008bd31e886a2fd335c2327a59e7d05e46cc43d8f7caae2ea78eee45849"
 Block Hash: "000f549267bee94aa5a36c2a21075c7b8ac70d83fd46eefd24a2e815f49f7"
 Nonce: 15693821
 Data: Ok("This is the first real block")

 Timestap: 1536840929
 Prev Block Hash: "000f549267bee94aa5a36c2a21075c7b8ac70d83fd46eefd24a2e815f49f7"
 Block Hash: "0001f568b7ebb5a18acbe1dceb2c0a3ef44206cd9dc1a2b3b3268dd5b0"
 Nonce: 19928433
 Data: Ok("And this is the next one")
```



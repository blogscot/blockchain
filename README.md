
## Intro

This repo is largely inspired by [asymmetric's blagh](https://asymmetric.github.io) blogpost [Implementing a Blockchain in Rust](https://asymmetric.github.io/2018/02/11/blockchain-rust/). I started hacking on the code snippets, eventually rewriting parts of the codebase. It's a good post, take a look.

### Running the code

This code was built and run using the nightly build, `rustc 1.30.0-nightly (90d36fb59 2018-09-13)`, along with the Rust `2018 Edition` enabled.

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

### Output Log

The output log below shows an example blockchain containing three blocks: the initial `genesis` block is a special block (containing solely dummy data) which has the sole purpose of starting a new chain. Each of the subsequent blocks contains a hashed value of previous block (providing a continuous encrypted link between blocks, hence the name _blockchain_). To find a valid hash value, that is, one that meets the required `DIFFICULTY` level, the `proof` value is iterated until a suitable hash value is found.

```
 Timestamp: 1536930780
 Prev Block Hash: "00000000000000000000000000000000"
 Proof: 0
 Data: Ok("Genesis block")

 Timestamp: 1536930781
 Prev Block Hash: "000f5f9f888a99a862982cf6517e7ff4bce2436b9c3ff5e395b57df7add"
 Proof: 1060038
 Data: Ok("This is the first real block")

 Timestamp: 1536930789
 Prev Block Hash: "0007c6c3eaac7cedacab1d6e79eee93ca3b29a2213cd52558cd72e57c6a8"
 Proof: 11347470
 Data: Ok("And this is the next one")
```

## Further Reading

[Blockchain Mining with Rust](https://www.innoq.com/en/blog/blockchain-mining-with-rust/)
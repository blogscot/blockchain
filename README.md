
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
const MAX_PROOF: u64 = u64::MAX;
```

A `DIFFICULTY` value affects how many leading zeroes each hash value is required to have. For example, using a value of 6 would provide 3 leading zeros, as we can see in the output below. 

As may be evident already, the higher the `DIFFICULTY` value the greater the number of iterations required to find a suitable hash value. The number of iterations is represented by the `Proof` value shown in the output below. Note, this value is used as a [nonce](https://en.wikipedia.org/wiki/Cryptographic_nonce) value during calculation of the previous block's hash value.

Using a release build, mining a new block with a `DIFFICULTY` value of 6 typically takes in the order of 5-20 seconds to complete.

### User Interface

```rust
fn run() -> Result<(), block::MiningError> {
    let mut chain = Blockchain::genesis();
    chain.add_block("This is the first real block")?;
    chain.add_block("And this is the next one")?;
    println!("{}", chain);
    Ok(())
}
```

### Output Log

The output log below shows an example blockchain containing three blocks: the initial `genesis` block is a special block (containing solely dummy data) which has the sole purpose of starting a new chain. Each of the subsequent blocks contains a hashed value of previous block, thus, providing a continuous encrypted link between blocks - hence the name _blockchain_. To find a valid hash value, that is, one that meets the required `DIFFICULTY` level, the `proof` value is iterated until a suitable hash value is found.

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

+ [Blockchains from Scratch](https://medium.com/mimir-blockchain/the-birds-the-bees-and-the-merkle-trees-ep-0-blockchains-from-scratch-3cedb1e669eb)

+ [Blockchain Mining with Rust](https://www.innoq.com/en/blog/blockchain-mining-with-rust/)
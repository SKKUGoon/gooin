# Rust Project Management

<p>
Note that this project is not for actually "making blockchain", 
but rather to familiarize myself with the concept of RUST.
</p>

# Notes

## 7.13
- File structuring
- When making multiple files, one should edit `Cargo.toml`, `lib.rs`, `main.rs`.

  - In `Cargo.toml` specify the name of the library in `[lib]` and main file as `[[bin]]`
  - If you need dependencies, write down like so
```toml
[lib]
name = "blockchainlib"
path = "src/lib.rs"

[[bin]]
name = "blockchain"
path = "src/main.rs"
```
```toml
[dependencies]
hex = "0.4.3"
```

- To export my code into `main.rs` -> it must first be imported to my lib.rs file
  - I created Block in block.rs like so. `lib.rs` should contain `mod block` and `pub use ...`

```rust
pub struct Block {
    pub index: u32,
    pub timestamp: u128,  // unix time
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,  // later becomes transaction 
}
```
```rust
mod block;
pub use crate::block::Block;
```

- Finally, `main.rs` should import all of the settings in `lib.rs` by calling the name of `[lib]` defined in `Cargo.toml`

```rust
use blockchainlib::*;
```
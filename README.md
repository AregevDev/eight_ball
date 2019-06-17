[![crates.io version]][crates.io link]
[![docs-badge][]][docs]

# eight_ball
A Rust crate mimicking the original 8 magic ball game.  
Ask a question and it will reply...

### Example
```rust
fn main() {
    let question = "Am I in charge?";
    println!("Q: {}\nA: {}", question, eight_ball::ask(question).content());
}
```

### Licence
Apache-2.0

[crates.io link]: https://crates.io/crates/eight_ball
[crates.io version]: https://img.shields.io/crates/v/eight_ball.svg?style=flat-square
[docs]: https://docs.rs/eight_ball
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
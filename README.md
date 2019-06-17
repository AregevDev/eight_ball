![crates.io(crates.io/crates/eight_ball)](https://img.shields.io/crates/v/eight_ball.svg?style=flat-square)
![docs.rs(docs.rs/eight_bal)](https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square)

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

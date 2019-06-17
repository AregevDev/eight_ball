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

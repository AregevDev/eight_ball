[![cratesio version]][cratesio link]
[![docs-badge][]][docs link]

# eight_ball
Mimics the original 8 magic ball game.

Ask a question and it will reply with an answer...  
Also supports predefinded custom answers.


### Example
```rust
use eight_ball::{Answer, AnswerType, EightBall};

fn main() {
    let question = "Am I in charge?";
    let answers = vec![
        Answer::new("UwU.", AnswerType::Affirmative),
        Answer::new("OwO.", AnswerType::NonCommittal),
        Answer::new("Baka No.", AnswerType::Negative),
    ];
    let b = EightBall::with_answers(answers);

    println!("{}", question);
    println!("{}", b.ask(question).unwrap().content);
}

```

### Licence
Apache-2.0

[cratesio link]: https://crates.io/crates/eight_ball
[cratesio version]: https://img.shields.io/crates/v/eight_ball.svg?style=flat-square

[docs link]: https://docs.rs/eight_ball
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square

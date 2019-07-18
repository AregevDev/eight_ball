//! Rust crate that mimics the original magic eight ball game.
//!
//! Ask a question and it will reply...  
//! Now supports custom answers

use rand::seq::SliceRandom;

/// The crate's entry point, holdding the current possible answers
pub struct EightBall {
    pub answers: Vec<Answer>,
}

impl EightBall {
    /// Constructs a new EightBall, the possibe answers are matching the original answers  
    pub fn new() -> Self {
        EightBall {
            answers: vec![
                // affirmative answers
                Answer::new("It is certain.", AnswerType::Affirmative),
                Answer::new("It is decidedly so.", AnswerType::Affirmative),
                Answer::new("Without a doubt.", AnswerType::Affirmative),
                Answer::new("Yes - definitely.", AnswerType::Affirmative),
                Answer::new("You may rely on it.", AnswerType::Affirmative),
                Answer::new("As I see it, yes.", AnswerType::Affirmative),
                Answer::new("Most likely.", AnswerType::Affirmative),
                Answer::new("Outlook good.", AnswerType::Affirmative),
                Answer::new("Yes.", AnswerType::Affirmative),
                Answer::new("Signs point to yes.", AnswerType::Affirmative),
                // non-committal answers
                Answer::new("Reply hazy, try again.", AnswerType::NonCommittal),
                Answer::new("Ask again later.", AnswerType::NonCommittal),
                Answer::new("Better not tell you now.", AnswerType::NonCommittal),
                Answer::new("Cannot predict now.", AnswerType::NonCommittal),
                Answer::new("Concentrate and ask again.", AnswerType::NonCommittal),
                // negative answers
                Answer::new("Don't count on it.", AnswerType::Negative),
                Answer::new("My reply is no.", AnswerType::Negative),
                Answer::new("My sources say no.", AnswerType::Negative),
                Answer::new("Outlook not so good.", AnswerType::Negative),
                Answer::new("Very doubtful.", AnswerType::Negative),
            ],
        }
    }

    /// Sets the possible answers the crate can return, this allows for custom answers
    pub fn with_answers(answers: Vec<Answer>) -> Self {
        EightBall { answers }
    }

    /// Asks the magic ball a question returns a possible answer randomly it picks a random answer from the possible ones
    pub fn ask(&self, _question: &str) -> Option<Answer> {
        if self.answers.is_empty() {
            return None;
        }

        Some(*self.answers.choose(&mut rand::thread_rng()).unwrap())
    }
}

/// The type of an answer, used to determine it's color in the future.
///
/// Can be either affirmative, non-committal or negative.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub enum AnswerType {
    Affirmative,
    NonCommittal,
    Negative,
}

impl Default for AnswerType {
    fn default() -> Self {
        AnswerType::Affirmative
    }
}

/// Represents a possible answer the magic ball can return.
///
/// Holds the content str and it's type
#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Answer {
    pub content: &'static str,
    pub answer_type: AnswerType,
}

impl Answer {
    /// Constructs a new Answer instance with a given content and type.
    pub const fn new(content: &'static str, answer_type: AnswerType) -> Self {
        Answer {
            content,
            answer_type,
        }
    }
}

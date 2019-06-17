use rand::seq::SliceRandom;

pub fn ask(_question: &'static str) -> Answer {
    *POSSIBLE_ANSWERS.choose(&mut rand::thread_rng()).unwrap()
}

static POSSIBLE_ANSWERS: [Answer; 20] = [
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
];

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

#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Answer {
    content: &'static str,
    answer_type: AnswerType,
}

impl Answer {
    pub const fn new(content: &'static str, answer_type: AnswerType) -> Self {
        Answer { content, answer_type }
    }
}

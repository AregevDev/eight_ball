#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub enum AnswerType {
    Affirmative,
    NonCommittal,
    Negative,
}

#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Answer {
    content: String,
    question_type: AnswerType,
}

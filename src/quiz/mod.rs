pub mod quiz_compiler;
pub mod question;

pub use question::{ask, build_question, Question};

mod validator;
use validator::validate_answer;

#[derive(Debug)]
pub struct Answer<'a> {
    pub question: &'a Question,
    pub answer: String,
}

pub fn show_result(answers: Vec<Answer>) {
    println!("\n---- Resultado ----");

    for answer in answers {
        validate_answer(&answer.answer, &answer.question.expected);
    }
}

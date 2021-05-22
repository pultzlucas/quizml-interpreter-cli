pub mod question;
pub use question::{Question, build_question, ask, validate_answer};

#[derive(Debug)]
pub struct Answer<'a> {
    pub question: &'a Question,
    pub answer: String
}


pub fn show_result(answers: Vec<Answer>){
    println!("\n---- Resultado ----");

    for answer in answers {
       validate_answer(&answer.answer, &answer.question.expected);
    }
}
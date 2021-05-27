pub mod interpreter;
use std::io::stdin;

pub use interpreter::{get_quiz_questions, Question};

mod validator;
use validator::validate_answer;

#[derive(Debug)]
pub struct Answer<'a> {
    pub question: &'a Question,
    pub answer: String,
}

pub fn show_result(questions: Vec<Question>) {
    
    let mut answers: Vec<Answer> = vec![];
    
    for q in questions.iter() {
        let answer = ask(&q);
        
        answers.push(Answer {
            question: q,
            answer: answer
        })
    }
    
    println!("\n---- Resultado ----");
    for answer in answers {
        validate_answer(&answer.answer, &answer.question.expected);
    }
}


pub fn ask(quest: &Question) -> String {
    println!("{}", quest.text);

    let mut answer = String::new();

    if let Err(e) = stdin().read_line(&mut answer) {
        println!("Oops! Something went wrong: {}", e);
    }

    answer
}

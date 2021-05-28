extern crate colored;

use colored::Colorize;
use std::io::stdin;

#[derive(Debug, PartialEq)]
pub struct Question {
    pub text: String,
    pub expected: String,
}

#[derive(Debug)]
struct Answer<'a> {
    question: &'a Question,
    answer: String,
}

pub fn show_result(questions: Vec<Question>) {
    let answers = get_answers(&questions);
    println!("\n---- Resultado ----");
    
    for answer in answers.iter() {
        validate_answer(&answer.answer, &answer.question.expected);
    }
}

fn get_answers<'a>(questions: &Vec<Question>) -> Vec<Answer> {
    let mut answers: Vec<Answer> = vec![];

    for q in questions.iter() {
        answers.push(Answer {
            question: q,
            answer: ask(&q),
        })
    }

    answers
}

fn ask(quest: &Question) -> String {
    println!("{}", quest.text);

    let mut answer = String::new();

    if let Err(e) = stdin().read_line(&mut answer) {
        println!("Oops! Something went wrong: {}", e);
    }

    answer
}

fn validate_answer(answer: &str, expected: &str) {
    let answer_is_correct = &answer.trim() == &expected.trim();

    match answer_is_correct {
        true => print_correct_res(answer),
        false => print_incorrect_res(answer)
    }
}

fn print_correct_res(answer: &str) {
    println!("Correto:   {} {}", answer.trim(), "v".green());
}
fn print_incorrect_res(answer: &str) {
    println!("Incorreto: {} {}", answer.trim(), "x".red());
}

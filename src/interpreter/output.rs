use std::io::{stdin, stdout, Write};

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
    println!("---- Resultado ----");
    for answer in answers.iter() {
        validate_answer(&answer.answer, &answer.question.expected);
    }
}

fn get_answers<'a>(questions: &Vec<Question>) -> Vec<Answer> {
    let mut answers: Vec<Answer> = vec![];

    for i in 0..questions.len() {
        answers.push(Answer {
            question: &questions[i],
            answer: ask(&questions[i], (i, questions.len())),
        })
    }

    answers
}

fn ask(quest: &Question, (question_i, total_i): (usize, usize)) -> String {
    let question_counter = format!("[{}/{}]", question_i + 1, total_i);

    println!("{} {}", question_counter, quest.text);
    print!("R: ");
    stdout().flush().unwrap();

    let mut answer = String::new();
    if let Err(e) = stdin().read_line(&mut answer) {
        println!("Oops! Something went wrong: {}", e);
    }

    println!("");

    answer
}

fn validate_answer(answer: &str, expected: &str) {
    let answer_is_correct = &answer.trim() == &expected.trim();

    match answer_is_correct {
        true => print_correct_res(answer),
        false => print_incorrect_res(answer),
    }
}

fn print_correct_res(answer: &str) {
    println!("Correto:   {} {}", answer.trim(), "✔");
}
fn print_incorrect_res(answer: &str) {
    println!("Incorreto: {} {}", answer.trim(), "✖");
}

use std::fs::File;
use std::io::{prelude::*, stdin, stdout, Error};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
struct Question {
    text: String,
    expected: String,
}

pub fn create_quiz() -> Result<String, Error> {
    let quiz_name = ask("Quiz name: ");
    let questions = get_questions();
    let questions_string = questions.iter().map(|q| get_question_string(q)).collect();

    write_quiz_file(quiz_name, questions_string)
}

fn write_quiz_file(quiz_name: String, questions: Vec<String>) -> Result<String, Error> {
    let file_name = format!("{}.quiz", quiz_name);
    let mut quiz_file = match File::create(file_name) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let quiz_string = questions.join(",\n");

    match quiz_file.write_all(quiz_string.as_bytes()) {
        Ok(_) => {},
        Err(e) => return Err(e)
    }

    Ok("Quiz file was created.".to_string())
}

fn get_question_string(question: &Question) -> String {
    format!("(\n\t-> {}\n\t<- {}\n)", question.text, question.expected)
}

fn get_questions() -> Vec<Question> {
    let mut questions: Vec<Question> = vec![];
    loop {
        let text = ask("Question text: ");
        let expected = ask("Question expected: ");

        questions.push(Question {
            text: text,
            expected: expected,
        });

        if ask("More Questions?(y/n) ") == "n" {
            break;
        }
    }

    questions
}

fn ask(text: &str) -> String {
    print!("{}", text);
    stdout().flush().unwrap();

    let mut info = String::new();
    if let Err(e) = stdin().read_line(&mut info) {
        println!("Oops! Something went wrong: {}", e);
    }

    info.trim().to_string()
}

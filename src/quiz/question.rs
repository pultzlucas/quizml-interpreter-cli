use std::io::stdin;

#[derive(Debug)]
pub struct Question {
    text: String,
    pub expected: String,
}

pub fn build_question(text: &str, expected: &str) -> Question {
    Question {
        text: String::from(text),
        expected: String::from(expected),
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

pub fn validate_answer(answer: &str, expected: &str) -> bool {
    let answer_is_correct = &answer.trim() == &expected.trim();

    if answer_is_correct {
        print!("Correto: {}", answer);
    } else {
        print!("Incorreto: {}", answer);
    }

    answer_is_correct
}

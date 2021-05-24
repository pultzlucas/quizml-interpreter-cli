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

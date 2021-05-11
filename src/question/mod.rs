#[derive(Debug)]
pub struct Question {
    text: String,
    expected: String,
}

pub fn ask(quest: Question) -> String {
    println!("{:?}", quest);

    quest.expected
}

pub fn build_question(text: &str, expected: &str) -> Question {
    Question {
        text: String::from(text),
        expected: String::from(expected),
    }
}

mod question;
use question::{ask, build_question};
fn main() {
    let text = "Qual o resultado da soma entre 50 e 51?";
    let expected = "101";
    
    let quest = build_question(text, expected);

    ask(quest);
}

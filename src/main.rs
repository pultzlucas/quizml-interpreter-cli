mod question;
use question::{ask, build_question, validate_answer};

fn main() {
    let quest1 = build_question("1) Qual o resultado da soma entre 50 e 51?", "101");
    let quest2 = build_question("2) Qual a capital do Brasil?", "Brasília");
    let quest3 = build_question("3) Qual a cor do céu?", "azul");

    let answer1: String = ask(&quest1);
    let answer2: String = ask(&quest2);
    let answer3: String = ask(&quest3);

    println!("\n---- Resultado ----");

    validate_answer(answer1, quest1.expected);
    validate_answer(answer2, quest2.expected);
    validate_answer(answer3, quest3.expected);
}

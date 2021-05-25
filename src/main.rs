mod quiz;
use quiz::{ask, build_question, show_result, Answer};

fn main() {
    let quest1 = build_question("1) Qual o resultado da soma entre 50 e 51?", "101");
    let quest2 = build_question("2) Qual a capital do Brasil?", "Brasília");
    let quest3 = build_question("3) Qual a cor do céu?", "azul");

    let questions = vec![
        Answer {
            question: &quest1,
            answer: ask(&quest1),
        },
        Answer {
            question: &quest2,
            answer: ask(&quest2),
        },
        Answer {
            question: &quest3,
            answer: ask(&quest3),
        },
    ];

    show_result(questions);
}

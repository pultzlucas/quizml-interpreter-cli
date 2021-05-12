use super::*;

#[test]
pub fn question1_expected() {
    assert_eq!(
        ask(build_question(
            "Qual o resultado da soma entre 50 e 51?",
            "101"
        )),
        "101"
    );
}

#[test]
pub fn question2_expected() {
    assert_eq!(
        ask(build_question("Qual a capital do Brasil?", "Brasília")),
        "Brasília"
    );
}

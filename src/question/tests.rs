use super::*;

#[test]
#[ignore]
pub fn question1_expected() {
    assert_eq!(
        ask(&build_question(
            "Qual o resultado da soma entre 50 e 51?",
            "101"
        )),
        "101"
    );
}

#[test]
#[ignore]
pub fn question2_expected() {
    assert_eq!(
        ask(&build_question("Qual a capital do Brasil?", "Brasília")),
        "Brasília"
    );
}

#[test]
pub fn clear_string_not_change() {
    assert_eq!(clear_string(&String::from("string")), "string");
}

#[test]
pub fn clear_string_change() {
    assert_eq!(clear_string(&String::from("\ns\ntring\n")), "string");
}

#[test]

pub fn test_answer_validation(){
    let answer = String::from("123");
    let expected = String::from("123");
    assert!(validate_answer(answer, expected));
}

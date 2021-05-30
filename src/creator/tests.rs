use super::*;

#[test]
#[ignore]
fn test_get_name() {
    assert_eq!(ask("oi:").trim(), "ola".to_string().trim());
}

#[test]
#[ignore]
fn test_get_question_string() {
    assert_eq!(
        get_question_string(&Question {
            text: String::from("Question text test"),
            expected: String::from("Question expected test")
        }),
        "(\n\t-> Question text test\n\t<- Question expected test\n)".to_string()
    );
}

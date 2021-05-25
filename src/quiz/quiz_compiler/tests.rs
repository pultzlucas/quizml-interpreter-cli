use super::*;

#[test]
//#[ignore]
fn test_get_questions_string() {
    assert_eq!(
        get_quiz_questions("test2"),
        vec![
            Question {
                text: String::from("tester2"),
                expected: String::from("123")
            },
            Question {
                text: String::from("blabla"),
                expected: String::from("albalblab")
            }
        ]
    )
}

#[test]
#[ignore]
fn test_regex() {
    let text = String::from("->tester2<-123");
    let regex = Regex::new(r"(?P<text>[^->]+)<-(?P<expected>[^<-]+)").unwrap();

    let captures = regex.captures(text.as_str()).unwrap();

    assert_eq!(
        vec![&captures["text"], &captures["expected"]],
        vec!["tester2", "123"]
    );
}

#[test]
#[ignore]
fn test_get_question_string_tuple() {
    assert_eq!(
        get_question_string_tuple(vec![
            String::from("(->tester2<-123)"),
            String::from("(->blabla<-albalblab)")
        ]),
        vec![
            (String::from("tester2"), String::from("123")),
            (String::from("blabla"), String::from("albalblab"))
        ]
    )
}

#[test]
#[ignore]
pub fn test_fs() {
    assert_eq!(get_file_string("test"), "(->tester<-123)")
}

#[test]
#[ignore]
pub fn test_fs2() {
    assert_eq!(
        get_file_string("test2"),
        "(->tester2<-123),(->blabla<-albalblab)"
    )
}

#[test]
#[ignore]
pub fn test_format_file_string() {
    assert_eq!(remove_spaces(String::from(" _oi \no\r")), "_oio")
}

#[test]
#[ignore]
pub fn test_value_regex() {
    assert_eq!(get_value("->   test_operator"), "test_operator")
}

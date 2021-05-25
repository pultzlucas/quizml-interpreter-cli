use std::fs;
extern crate regex;
use regex::Regex;

#[cfg(test)]
mod tests;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Question {
    text: String,
    expected: String,
}

pub fn get_quiz_questions(name: &str) -> Vec<Question> {
    let file_string = get_file_string(name);
    let questions_string = split_questions_string(file_string);
    let questions_tuple_vec = get_question_string_tuple(questions_string);

    let mut questions: Vec<Question> = vec![];
    for (text, expected) in questions_tuple_vec.iter() {
        questions.push(Question {
            text: String::from(text),
            expected: String::from(expected)
        })
    }

    questions
}

fn get_value(line: &str) -> String {
    let regex = Regex::new(r"(<- *|-> *)").unwrap();
    let capture = match regex.captures(line) {
        Some(caps) => String::from(&caps[0]),
        None => String::from(line),
    };

    let value = line.replace(capture.as_str(), "");

    value
}

fn split_questions_string(file_string: String) -> Vec<String> {
    file_string
        .split(",")
        .map(|string| String::from(string))
        .collect()
}

fn get_question_string_tuple(question_strings: Vec<String>) -> Vec<(String, String)> {
    let mut tup_array: Vec<(String, String)> = vec![];

    for qs in question_strings.iter() {
        let regex = Regex::new(r"(?P<text>[^->]+)<-(?P<expected>[^<-]+)\)").unwrap();
        let captures = regex.captures(qs.as_str()).expect("Interpretation error");

        tup_array.push((
            String::from(&captures["text"]),
            String::from(&captures["expected"]),
        ));
    }

    tup_array
}

fn get_file_string(name: &str) -> String {
    let base_path = "./src/quizes/";
    let file_path = format!("{}/{}.quiz", base_path, name);
    let data = fs::read_to_string(file_path).unwrap();
    remove_spaces(data)
}

fn remove_spaces(file_string: String) -> String {
    let regex = Regex::new(r"\S").unwrap();
    let mut string = file_string;
    string.retain(|c| regex.is_match(String::from(c).as_str()));
    string
}

use std::fs;
extern crate regex;
use regex::Regex;

mod output;
pub use output::show_result;
use output::Question;


#[cfg(test)]
mod tests;

pub fn get_quiz_questions(file_name: &str) -> Vec<Question> {
    let file_string = get_file_string(file_name);
    let questions_string = split_questions_string(file_string);
    let questions_tuple_vec = get_question_string_tuple(questions_string);

    let mut questions: Vec<Question> = vec![];
    for (text, expected) in questions_tuple_vec.iter() {
        questions.push(Question {
            text: String::from(text),
            expected: String::from(expected),
        })
    }

    questions
}

fn split_questions_string(file_string: String) -> Vec<String> {
    file_string
        .split(",")
        .filter(|string| *string != "")
        .map(|string| String::from(string))
        .collect()
}

fn get_question_string_tuple(question_strings: Vec<String>) -> Vec<(String, String)> {
    let mut tup_array: Vec<(String, String)> = vec![];

    for qs in question_strings.iter() {
        let regex = Regex::new(r"(?P<text>[^->]+)<-(?P<expected>[^<-]+)\)").unwrap();
        let captures = regex.captures(qs.as_str()).expect("Interpretation error");
        tup_array.push((
            String::from(captures["text"].trim()),
            String::from(captures["expected"].trim()),
        ));
    }

    tup_array
}

fn get_file_string(file_name: &str) -> String {
    let data = fs::read_to_string(file_name).expect("Error when read quiz file");
    remove_spaces(data)
}

fn remove_spaces(file_string: String) -> String {
    let regex = Regex::new(r"\n|\t|\r").unwrap();
    let mut string = file_string;
    string.retain(|c| !regex.is_match(String::from(c).as_str()));
    string
}

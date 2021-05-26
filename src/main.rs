mod quiz;
use quiz::{get_quiz_questions, show_result};
use std::env;

fn main() {
    let raw_args = env::args().collect::<Vec<String>>();
    let args = &raw_args[1..];

    if args.len() > 0 {
        let quiz_file_path = &args[0];
        let questions = get_quiz_questions(quiz_file_path);
        show_result(questions);
    }
}

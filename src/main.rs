mod interpreter;
use interpreter::{get_quiz_questions, show_result};
use std::{env, path::Path};

fn main() {
    let raw_args = env::args().collect::<Vec<String>>();
    let args = &raw_args[1..];

    if args.len() > 0 {
        let quiz_file_path = get_quiz_file_path(&args[0]);
        let questions = get_quiz_questions(quiz_file_path.as_str());
        show_result(questions);
    } else {
        println!("Please type the quiz file name.")
    }
}

fn get_quiz_file_path(filename: &String) -> String {
    let current_dir = env::current_dir().unwrap();
    let path = Path::new(&current_dir)
        .join(filename)
        .into_os_string()
        .into_string()
        .unwrap();
    path
}

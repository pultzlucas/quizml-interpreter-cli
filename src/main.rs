mod quiz;
use quiz::{get_quiz_questions, show_result};
use std::{env, path::Path};

fn main() {
    let raw_args = env::args().collect::<Vec<String>>();
    let args = &raw_args[1..];
    let current_dir = env::current_dir().expect("Dir error");

    if args.len() > 0 {
        let quiz_file_path = Path::new(&current_dir)
            .join(&args[0])
            .into_os_string()
            .into_string()
            .unwrap();
        let questions = get_quiz_questions(quiz_file_path.as_str());
        show_result(questions);
    } else {
        println!("Please type the quiz file name.")
    }
}

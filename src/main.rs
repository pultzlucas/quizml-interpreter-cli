mod interpreter;
use interpreter::{get_quiz_questions, show_result};
use std::{env, path::Path};

fn main() {
    let raw_args = env::args().collect::<Vec<String>>();
    let args = &raw_args[1..];

    if args.len() < 1 {
        println!("Please type the quiz file name.");
        return;
    }

    let quiz_file_path = match get_quiz_file_path(&args[0]) {
        Err(e) => return println!("{}", e),
        Ok(s) => s,
    };

    let questions = get_quiz_questions(quiz_file_path.as_str());
    show_result(questions);
}

fn get_quiz_file_path(filename: &String) -> Result<String, String> {
    let current_dir = match env::current_dir() {
        Ok(ok) => ok,
        Err(_) => return Err("Invalid current dir".to_string()),
    };

    let path = Path::new(&current_dir).join(filename);
    if !path.exists() {
        return Err(format!("Quiz file '{}' not exists.", filename));
    }

    Ok(path.into_os_string().into_string().unwrap())
}

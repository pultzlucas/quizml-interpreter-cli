use std::fs;

#[derive(Debug)]
pub struct Question {
    text: String,
    expected: String,
}

fn get_file_string(name: &str) -> String {
    let base_path = "./src/quizes/";
    let file_path = format!("{}/{}.quiz", base_path, name);
    fs::read_to_string(file_path).unwrap()
}

fn get_file_lines(file_string: String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    for line in file_string.split("\n") {
        let line_trimmed = String::from(line.trim());

        if line_trimmed != "" {
            lines.push(line_trimmed);
        }
    }

    lines
}

fn get_questions(name: &str) -> Vec<String> {
    let file_string = get_file_string(name);
    let lines = get_file_lines(file_string);
    lines
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_fs() {
        assert_eq!(get_file_string("test"), "-> tester\r\n<- 123")
    }

    #[test]
    //#[ignore]
    fn test_fs2() {
        assert_eq!(get_file_string("test2"), "-> tester2\r\n<- 123\r\n\r\n-> blabla\r\n<- albalblab")
    }

    #[test]
    #[ignore]
    fn test_lines_vec() {
        let data = String::from("-> tester\r\n<- 123");
        assert_eq!(get_file_lines(data), vec!["-> tester", "<- 123"])
    }

    #[test]
    //#[ignore]
    fn test_lines_vec2() {
        let data = String::from("-> tester2\r\n<- 123\r\n\r\n-> blabla\r\n<- albalblab");
        assert_eq!(get_file_lines(data), vec!["-> tester2", "<- 123", "-> blabla", "<- albalblab"])
    }
    #[test]
    #[ignore]
    fn test_get_questions() {
        assert_eq!(get_questions("test"), vec!["-> tester", "<- 123"])
    }
}

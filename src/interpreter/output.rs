use std::io::stdin;

#[derive(Debug, PartialEq)]
pub struct Question {
    pub text: String,
    pub expected: String,
}

#[derive(Debug)]
struct Answer<'a> {
    question: &'a Question,
    answer: String,
}

pub fn show_result(questions: Vec<Question>) {
    
    let mut answers: Vec<Answer> = vec![];
    
    for q in questions.iter() {
        let answer = ask(&q);
        
        answers.push(Answer {
            question: q,
            answer: answer
        })
    }
    
    println!("\n---- Resultado ----");
    for answer in answers {
        validate_answer(&answer.answer, &answer.question.expected);
    }
}


fn ask(quest: &Question) -> String {
    println!("{}", quest.text);

    let mut answer = String::new();

    if let Err(e) = stdin().read_line(&mut answer) {
        println!("Oops! Something went wrong: {}", e);
    }

    answer
}

fn validate_answer(answer: &str, expected: &str) -> bool {
    let answer_is_correct = &answer.trim() == &expected.trim();

    if answer_is_correct {
        print!("Correto: {}", answer);
    } else {
        print!("Incorreto: {}", answer);
    }

    answer_is_correct
}

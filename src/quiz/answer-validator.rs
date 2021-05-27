pub fn validate_answer(answer: &str, expected: &str) -> bool {
    let answer_is_correct = &answer.trim() == &expected.trim();

    if answer_is_correct {
        print!("Correto: {}", answer);
    } else {
        print!("Incorreto: {}", answer);
    }

    answer_is_correct
}
# QuizML Interpreter CLI

QuizML is a markup language to write quizes.

Basic example:

**questions.quiz**

````quiz
(
    -> What is the som among 1 plus 2?
    <- 3
),

(
    -> What is the largest ocean on earth?
    <- Pacific Ocean
)
````

To execute the question.quiz file type **quiz** and the name of file:

````console
quiz question.quiz
````
struct Flashcard {
    question: String,
    answer: String,
}

fn main() {
    let flashcards = vec![
        Flashcard {
            question: String::from("What is the capital of France?"),
            answer: String::from("Paris"),
        },
        Flashcard {
            question: String::from("What is 2 + 2?"),
            answer: String::from("4"),
        },
    ];

    for flashcard in flashcards {
        println!("{}", flashcard.question);
        let mut user_answer = String::new();
        std::io::stdin()
            .read_line(&mut user_answer)
            .expect("Failed to read user input");

        if user_answer.trim() == flashcard.answer {
            println!("Correct!");
        } else {
            println!("Incorrect. The correct answer is: {}", flashcard.answer);
        }
    }
}

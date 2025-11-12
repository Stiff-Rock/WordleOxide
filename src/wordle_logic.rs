use std::{
    cell::LazyCell,
    collections::HashMap,
    io::{Write, stdin, stdout},
    usize,
};

const INIT_BOARD: LazyCell<Vec<String>> = LazyCell::new(|| vec!["XXXXX".to_string(); 6]);

const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";

pub fn start_game(wordle: String) {
    let mut board: Vec<String> = INIT_BOARD.clone();

    let has_guessed: bool;

    let mut tries: usize = 0;

    println!();

    loop {
        print_board(&board);

        let mut attempt: String = String::new();

        // TODO: Maybe check in the web's dictionary if the word exists
        print!("Your guess: ");
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut attempt)
            .expect("Failed to read input");

        attempt = attempt.trim().to_uppercase();

        if attempt == wordle {
            has_guessed = true;
            break;
        }

        compare_attempt(&attempt, &wordle, &tries, &mut board);

        tries += 1;

        if tries >= 6 {
            has_guessed = false;
            break;
        }

        println!();
    }

    println!();
    if has_guessed {
        println!("Congratulations!!")
    } else {
        println!("The word was \"{wordle}\"")
    }
}

fn print_board(board: &Vec<String>) {
    for row in board {
        println!("{row}");
    }
}

fn compare_attempt(attempt: &String, wordle: &String, tries: &usize, board: &mut Vec<String>) {
    let mut new_letters_row: String = String::new();

    let mut chars_frequency: HashMap<char, usize> = HashMap::new();
    for wordle_char in wordle.chars() {
        *chars_frequency.entry(wordle_char).or_insert(0) += 1;
    }

    // TODO: IT IS NOT PERFECT. IF WORD IS 'DEUCE' AND YOU INPUT 'EEEEE' IT WILL LOOK LIKE 'E(Y when it shouln't be)|E(G)|E(X)|E(X)|E(X when it should be G)'
    for (attempt_char, wordle_char) in attempt.chars().zip(wordle.chars()) {
        let char_freq_count = chars_frequency.get(&attempt_char).unwrap_or(&0);

        if *char_freq_count > 0 {
            chars_frequency
                .entry(attempt_char)
                .and_modify(|count| *count -= 1);

            if attempt_char == wordle_char {
                let exact_letter = format!("{}{}{}", GREEN, attempt_char, RESET);
                new_letters_row.push_str(&exact_letter);
            } else {
                let present_letter: String = format!("{}{}{}", YELLOW, attempt_char, RESET);
                new_letters_row.push_str(&present_letter);
            }
        } else {
            new_letters_row.push_str(&attempt_char.to_string());
        }
    }

    board[*tries] = new_letters_row;
}

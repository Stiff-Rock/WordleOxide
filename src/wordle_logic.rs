use std::{
    cell::LazyCell,
    collections::{HashMap, HashSet},
    io::{Write, stdin, stdout},
    usize,
};

const INIT_BOARD: LazyCell<Vec<String>> = LazyCell::new(|| vec!["XXXXX".to_string(); 6]);

const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";

pub fn start_game(wordle: String, dict: HashSet<String>) {
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

        if !dict.contains(&attempt.to_lowercase()) {
            println!("{}Invalid word{}\n", RED, RESET);
            continue;
        }

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
        let win_msg = match tries {
            0 => "Genius!",
            1 => "Magnificent!",
            2 => "Impressive!",
            3 => "Splendid!",
            4 => "Great!",
            5 => "Phew...",
            _ => "Wtf?",
        };

        println!("{win_msg}\nYou guessed the word!!")
    } else {
        println!("Game Over!\nThe word was \"{wordle}\"")
    }
}

fn print_board(board: &Vec<String>) {
    for row in board {
        println!("{row}");
    }
}

fn compare_attempt(attempt: &String, wordle: &String, tries: &usize, board: &mut Vec<String>) {
    let mut new_letters_row: Vec<String> = Vec::new();

    let mut chars_frequency: HashMap<char, usize> = HashMap::new();
    for wordle_char in wordle.chars() {
        *chars_frequency.entry(wordle_char).or_insert(0) += 1;
    }

    let attempt_chars: Vec<char> = attempt.chars().collect();
    let wordle_chars: Vec<char> = wordle.chars().collect();
    let len = attempt_chars.len();

    for i in 0..len {
        let char_freq_count = chars_frequency.get(&attempt_chars[i]).unwrap_or(&0);

        if attempt_chars[i] == wordle_chars[i] {
            let exact_letter = format!("{}{}{}", GREEN, attempt_chars[i], RESET);

            if *char_freq_count > 0 {
                chars_frequency
                    .entry(attempt_chars[i])
                    .and_modify(|count| *count -= 1);
            }

            new_letters_row.push(exact_letter);
        } else {
            new_letters_row.push("-".to_string());
        }
    }

    for i in 0..len {
        if new_letters_row[i] != "-".to_string() {
            continue;
        }

        let char_freq_count = chars_frequency.get(&attempt_chars[i]).unwrap_or(&0);

        if *char_freq_count > 0 {
            chars_frequency
                .entry(attempt_chars[i])
                .and_modify(|count| *count -= 1);

            let present_letter: String = format!("{}{}{}", YELLOW, attempt_chars[i], RESET);
            new_letters_row[i] = present_letter;
        } else {
            new_letters_row[i] = attempt_chars[i].to_string();
        }
    }

    board[*tries] = new_letters_row.join("");
}

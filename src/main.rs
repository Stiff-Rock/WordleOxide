mod en_wordle;
use arboard::Clipboard;
use num_format::{Locale, ToFormattedString};
mod es_wordle;
mod ntp;
mod wordle_logic;
use std::{
    char,
    io::{Write, stdin, stdout},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut option_num;

    loop {
        print_options();

        loop {
            let mut user_select = String::new();

            stdin().read_line(&mut user_select).unwrap();

            option_num = user_select.trim().parse::<i32>().unwrap_or(-1);

            if option_num < 1 || option_num > 3 {
                eprintln!("Invalid input, please enter a valid number");
            } else {
                break;
            }
        }

        let (game_name, word_num_text, wordle, dict) = if option_num == 1 {
            let data: en_wordle::NytApiResponse = en_wordle::get_daily_word()
                .map_err(|e| format!("Unable to get today's wordle: {e}"))?;
            let dict = en_wordle::get_word_dictionary().unwrap();

            let word = data.solution.to_uppercase();
            let word_num_text = data.days_since_launch.to_formatted_string(&Locale::en);

            ("Wordle", word_num_text, word, dict)
        } else if option_num == 2 {
            let data = es_wordle::get_daily_word()
                .map_err(|e| format!("Unable to get today's wordle: {e}"))?;
            let dict = es_wordle::get_word_dictionary().unwrap();

            let word = data.0;
            let word_num_text = format!("#{}", data.1);

            ("La palabra del día", word_num_text, word, dict)
        } else {
            break;
        };

        let final_board = wordle_logic::start_game(game_name, word_num_text, wordle, dict);

        loop {
            println!("\nWrite 'c' to copy the result");
            println!("Press ENTER to go back to menu...");
            let mut action = String::new();
            stdin().read_line(&mut action).unwrap();

            if action.trim().to_lowercase() == "c" {
                copy_result(&final_board)?;
            } else {
                break;
            }
        }
    }

    Ok(())
}

fn print_options() {
    println!("\n-- WordleOxide --\n");
    println!("1. NYT Wordlde");
    println!("2. LaPalabraDelDía");
    println!("3. Exit");
    print!("Selection: ");
    stdout().flush().unwrap();
}

fn copy_result(final_board: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut copied_board = String::new();
    let first_row = &final_board[0];

    let first_row_jump = first_row.to_string() + "\n\n";
    copied_board.push_str(&first_row_jump);

    let mut finish = false;
    for i in 1..final_board.len() {
        let row = &final_board[i];
        let row_chars: Vec<char> = row.chars().collect();

        let mut j: usize = 0;
        let row_len = row.len();
        while j < row_len {
            let current_char = row_chars[j];

            match current_char {
                'X' => {
                    finish = true;
                    break;
                }
                '\x1b' => {
                    j += 2;
                }
                '3' => {
                    match row_chars[j + 1] {
                        '2' => copied_board.push_str("🟩"),
                        '3' => copied_board.push_str("🟨"),
                        _ => (),
                    }
                    j += 4;
                }
                '0' => {
                    j += 2;
                }
                _ => {
                    copied_board.push_str("⬛");
                    j += 1;
                }
            }
        }

        if finish {
            break;
        }

        copied_board.push_str("\n");
    }

    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(copied_board)?;

    println!("Result copied to the clipboard!");

    Ok(())
}

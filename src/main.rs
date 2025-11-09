mod en_wordle;
mod es_wordle;
mod ntp;
mod wordle_logic;
use std::io::{Write, stdin, stdout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut option_num;

    loop {
        print_options();

        loop {
            let mut user_select = String::new();

            stdin()
                .read_line(&mut user_select)
                .expect("Failed to read input");

            option_num = user_select.trim().parse::<i32>().unwrap_or(-1);

            if option_num < 1 || option_num > 3 {
                eprintln!("Invalid input, please enter a valid number");
            } else {
                break;
            }
        }

        let wordle = if option_num == 1 {
            en_wordle::get_daily_word().map_err(|e| format!("Unable to get today's wordle: {e}"))?
        } else if option_num == 2 {
            es_wordle::get_daily_word().map_err(|e| format!("Unable to get today's wordle: {e}"))?
        } else {
            break;
        };

        wordle_logic::start_game(wordle);

        pause();
    }

    Ok(())
}

pub fn print_options() {
    println!("\n-- WordleOxide --\n");
    println!("1. NYT Worlde");
    println!("2. LaPalabraDelDía");
    println!("3. Exit");
    print!("Selection: ");
    stdout().flush().unwrap();
}

pub fn pause() {
    print!("\nPress ENTER to go back to menu...");
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut String::new());
}

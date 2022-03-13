use colored::*;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::io;

#[derive(Deserialize, Debug)]
struct GuessSlot {
    guess: String,
    result: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("\nWordle in your terminal! 🔠 🟩 🟨 🟥");

    let mut tries_left = 6;
    let mut did_win = false;
    let mut solutions: Vec<[GuessSlot; 5]> = vec![];

    println!("You have 6 chances to guess the right 5 letter word of the day.");
    println!("Good luck! 🙆\n");

    loop {
        if tries_left == 0 && !did_win {
            println!("\nYou lost... 😔");
            println!("Wanna try again? [Y/N]\n");

            let mut again = String::new();
            io::stdin().read_line(&mut again).expect("Wrong input");

            let as_char: Vec<char> = again.to_lowercase().chars().collect();

            if as_char[0] == 'y' {
                solutions.clear();
                tries_left = 6;
                println!("\n⚡️ Let's go!");
                println!("----------------\n");
                continue;
            } else {
                println!("\nSee ya!");
                println!("PS: You gotta play again to reveal the secret word. 😉\n");
                break;
            }
        } else if did_win {
            println!("\nYou won! Congratulations! 🎉");
            println!("See you tomorrow! 🙋\n");
            break;
        }

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Faulty input.");

        if guess.len() - 1 < 5 || guess.len() - 1 > 5 {
            println!("\nYou need to insert 5 characters!");
            println!("You typed {} characters.", guess.len() - 1);
            println!("Try again!");
            println!("----------------\n");
            continue;
        }

        let resp =
            reqwest::blocking::get(format!("https://v1.wordle.k2bd.dev/daily?guess={}", guess))?
                .text()?;

        let split_guess = guess.split("").collect::<Vec<&str>>();
        let mut filtered_split_guess = split_guess
            .into_iter()
            .filter(|x| x.to_owned() != "".to_owned())
            .collect::<Vec<&str>>();
        filtered_split_guess.pop();

        let result: [GuessSlot; 5] = serde_json::from_str(&resp).unwrap();
        did_win = determine_win(&result);

        solutions.push(result);
        print_rows(&solutions);

        tries_left -= 1;
    }
    Ok(())
}

fn print_rows(solutions: &Vec<[GuessSlot; 5]>) {
    for solution in solutions {
        for slot in solution {
            print!(
                "[{}]",
                match slot.result.as_str() {
                    "correct" => slot.guess.to_string().to_uppercase().green(),
                    "present" => slot.guess.to_string().to_uppercase().yellow(),
                    "absent" => slot.guess.to_string().to_uppercase().red(),
                    _ => slot.guess.to_string().to_uppercase().white(),
                }
            );
        }
        print!("\n");
    }
    println!("----------------");
}

fn determine_win(guess: &[GuessSlot; 5]) -> bool {
    let mut result = false;
    let mut acc = 0;

    for slot in guess {
        match slot.result.as_str() {
            "correct" => acc += 1,
            _ => continue,
        }
    }

    if acc == 5 {
        result = true
    }

    result
}

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
    println!("🔠 Wordle in your terminal!");

    let mut tries_left = 6;
    let did_win = false;
    let mut solutions: Vec<[GuessSlot; 5]> = vec![];

    println!("Start guessing:");

    loop {
        if tries_left == 0 && !did_win {
            println!("You lost");
            break;
        } else if did_win {
            print_rows(&solutions);
            println!("You won! 🎉");
        }

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Faulty input.");

        if guess.len() - 1 < 5 || guess.len() - 1 > 5 {
            println!("You need to insert 5 characters!");
            println!("You types {} characters.", guess.len() - 1);
            println!("Game over...");
            break;
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

use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::io;

#[derive(Deserialize, Debug)]
struct GuessSlot {
    slot: u8,
    guess: String,
    result: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Wordle in your terminal!");
    loop {
        println!("Enter your guess!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Faulty input.");

        let resp =
            reqwest::blocking::get(format!("https://v1.wordle.k2bd.dev/daily?guess={}", guess))?
                .text()?;

        let result: [GuessSlot; 5] = serde_json::from_str(&resp).unwrap();
        println!("{:?}", result);
        break;
    }
    Ok(())
}

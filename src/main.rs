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

        let split_guess = guess.split("").collect::<Vec<&str>>();
        let mut filtered_split_guess = split_guess
            .into_iter()
            .filter(|x| x.to_owned() != "".to_owned())
            .collect::<Vec<&str>>();
        filtered_split_guess.pop();
        println!("{:?}", filtered_split_guess);
        let result: [GuessSlot; 5] = serde_json::from_str(&resp).unwrap();
        println!("{:?}", result);
        break;
    }
    Ok(())
}
